use bitvec::{field::BitField as _, order::Lsb0, view::BitView};
use nobcd::BcdNumber;

use crate::prelude::internal::*;

/// Parses the given 18-byte array for a range limits descriptor block.
///
/// Note that the `edid` input needs to be a full EDID.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8; 18], edid: &[u8]) -> Result<RangeLimitsDesc, EdidError> {
    let limits = just_limits(input)?;

    // all gtf-complaint displays are continuous frequency.
    // therefore, if something reports to use gtf, check that this is true!
    let cf = edid[0x18].view_bits::<Lsb0>()[0];
    let check_supports_cont_freq = || {
        if !cf {
            tracing::warn!("The EDID reported supporting GTF, but its feature support bit is off!");
        }
    };

    Ok(match input[10] {
        // gtf
        0x00 => {
            check_supports_cont_freq();
            RangeLimitsDesc::GtfSupported { limits }
        }

        // just range limits.
        0x01 => RangeLimitsDesc::LimitsOnly {
            limits,
            flexible: cf,
        },

        // secondary gtf supported
        0x02 => {
            check_supports_cont_freq();
            gtf_secondary_curve(limits, input)
        }
        0x04 => {
            check_supports_cont_freq();
            cvt(limits, input)?
        }
        reserved => {
            tracing::error!(
                "The given descriptor used a reserved video timing support flag! (`reserved`)"
            );
            return Err(EdidError::DescriptorRangeLimitsUsedReservedVTSFlag { flag: reserved });
        }
    })
}

/// For when byte 10 == 0x02.
#[tracing::instrument]
fn gtf_secondary_curve(limits: RangeLimits, input: &[u8; 18]) -> RangeLimitsDesc {
    if input[11] != 0x00 {
        tracing::warn!(
            "GTF Secondary Curve reserved byte was non-zero! (byte: {})",
            input[11]
        );
    }

    let start_break_freq = input[12] as u16 * 2;
    let c2 = input[13];
    let m: u16 = [input[14], input[15]].view_bits::<Lsb0>().load();
    let k = input[16];
    let j2 = input[17];

    RangeLimitsDesc::GtfSecondaryCurveSupported {
        limits,
        start_break_freq,
        c2,
        m,
        k,
        j2,
    }
}

/// For when byte 10 == 0x04.
#[tracing::instrument]
fn cvt(limits: RangeLimits, input: &[u8; 18]) -> Result<RangeLimitsDesc, EdidError> {
    let cvt_version = input[11];
    let enhanced_px_clk = {
        let to_sub: u8 = input[12].view_bits::<Lsb0>()[2..=7].load();
        let decimal = Decimal::from(limits.max_pixel_clock_mhz);
        decimal - (Decimal::from(to_sub) / Decimal::from(4))
    };

    let maximum_active_pxls_per_line = {
        let lower = input[13];

        // if byte 13 is all 0s, there's no limit!
        if lower == 0x00 {
            None
        } else {
            let upper: u16 = input[12].view_bits::<Lsb0>()[0..=1].load::<u16>() * 256_u16;
            let combined = upper + lower as u16;

            Some(combined * 8)
        }
    };

    let supported_aspect_ratios = {
        let bits = input[14].view_bits::<Lsb0>();

        if !bits[0..=2].iter().all(|b| b == false) {
            tracing::warn!("Reserved bits for supported aspect ratios are in use.");
        }

        SupportedAspectRatios {
            _4x3: bits[7],
            _16x9: bits[6],
            _16x10: bits[7],
            _5x4: bits[4],
            _15x9: bits[3],
        }
    };

    let preferred_aspect_ratio = {
        let bits = input[15].view_bits::<Lsb0>();
        match (bits[7], bits[6], bits[5]) {
            (false, false, false) => PreferredAspectRatio::_4x3,
            (false, false, true) => PreferredAspectRatio::_16x9,
            (false, true, false) => PreferredAspectRatio::_16x10,
            (false, true, true) => PreferredAspectRatio::_5x4,
            (true, false, false) => PreferredAspectRatio::_15x9,
            other => {
                tracing::error!(
                    "Used reserved combination of pref. aspect ratio bits. ({other:x?})"
                );
                return Err(EdidError::DescriptorRangeLimitsCvtReservedBits);
            }
        }
    };

    let (supports_standard_cvt_blanking, supports_reduced_cvt_blanking) = {
        let bits = input[15].view_bits::<Lsb0>();
        (bits[3], bits[4])
    };

    let (
        supports_h_shrink_scaling,
        supports_h_stretch_scaling,
        supports_v_shrink_scaling,
        supports_v_stretch_scaling,
    ) = {
        let bits = input[16].view_bits::<Lsb0>();
        (bits[7], bits[6], bits[5], bits[4])
    };

    let preferred_v_refresh_rate_hz = input[17];

    Ok(RangeLimitsDesc::CvtSupported {
        limits,
        enhanced_px_clk,
        cvt_version,
        maximum_active_pxls_per_line,
        supported_aspect_ratios,
        preferred_aspect_ratio,
        supports_standard_cvt_blanking,
        supports_reduced_cvt_blanking,
        supports_h_shrink_scaling,
        supports_h_stretch_scaling,
        supports_v_shrink_scaling,
        supports_v_stretch_scaling,
        preferred_v_refresh_rate_hz,
    })
}

#[tracing::instrument(skip_all)]
fn just_limits(input: &[u8; 18]) -> Result<RangeLimits, EdidError> {
    // we need these to calculate the min/max rates
    let offsets = limit_offsets(input[4])?;

    let get_rate = |idx| -> Result<u16, EdidError> {
        let val = from_bcd(input[idx])?;
        if val == 0x0 {
            tracing::warn!("a min/max rate incorrectly has a value of zero! (at `input[{idx}]`)");
        }
        Ok(val)
    };

    let min_vt = {
        let mut val = get_rate(5)?;
        if offsets.vertical.has_min() {
            val += 255;
        }
        val
    };

    let max_vt = {
        let mut val = get_rate(6)?;
        if offsets.vertical.has_max() {
            val += 255;
        }
        val
    };

    let min_hz = {
        let mut val = get_rate(7)?;
        if offsets.horizontal.has_min() {
            val += 255;
        }
        val
    };

    let max_hz = {
        let mut val = get_rate(8)?;
        if offsets.horizontal.has_max() {
            val += 255;
        }
        val
    };

    let pixel_clock = from_bcd(input[9])? * 10;

    Ok(RangeLimits {
        min_v_rate_hz: min_vt,
        max_v_rate_hz: max_vt,
        min_h_rate_khz: min_hz,
        max_h_rate_khz: max_hz,
        offsets,
        max_pixel_clock_mhz: pixel_clock,
    })
}

#[tracing::instrument]
fn limit_offsets(byte: u8) -> Result<Offsets, EdidError> {
    let bits = byte.view_bits::<Lsb0>();

    // error if the reserved bits are used
    let rev_bits = &bits[4..=7];
    if !rev_bits.iter().all(|b| b == false) {
        tracing::error!("Reserved display range limit offset bits were used: {rev_bits:x?}");
        return Err(EdidError::DescriptorRangeLimitsUsedReservedBits { input: byte });
    }

    // otherwise, we got some offsets

    let hz_bits = (bits[3], bits[2]);
    let horizontal = match hz_bits {
        (false, false) => HorizontalOffset::Zero,
        (true, false) => HorizontalOffset::Max255kHz_MinNotOffset,
        (true, true) => HorizontalOffset::Max255kHz_Min255kHz,

        _ => {
            tracing::error!("Horizontal bytes were weird: {hz_bits:#?}");
            return Err(EdidError::DescriptorRangeLimitsUsedReservedBits { input: byte });
        }
    };

    let vt_bits = (bits[1], bits[0]);
    let vertical = match vt_bits {
        (false, false) => VerticalOffset::Zero,
        (true, false) => VerticalOffset::Max255Hz_MinNotOffset,
        (true, true) => VerticalOffset::Max255Hz_Min255Hz,

        _ => {
            tracing::error!("Horizontal bytes were weird: {hz_bits:#?}");
            return Err(EdidError::DescriptorRangeLimitsUsedReservedBits { input: byte });
        }
    };

    Ok(Offsets {
        horizontal,
        vertical,
    })
}

/// Wraps the BCD-encoded number in a helpful type.
#[tracing::instrument]
fn typed_bcd(input: u8) -> Result<BcdNumber<2>, EdidError> {
    let bcd = BcdNumber::new(input).map_err(|e| {
        tracing::error!("Failed to create BcdNumber. (err: {e:?})");
        EdidError::BcdError
    });
    tracing::debug!("made typed bcd (from: `{input}`, to: `{bcd:?}`)");
    bcd
}

/// Makes a typical number from the given BCD-encoded one.
#[tracing::instrument]
fn from_bcd(input: u8) -> Result<u16, EdidError> {
    let val = typed_bcd(input)?.value();
    tracing::debug!("converted bcd (from: `{input}`, to: `{val}`)");
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _sam02e3_2c47316eff13_range_limits() {
        logger();
        let path = "linuxhw_edid_EDID_Digital_Samsung_SAM02E3_2C47316EFF13.input";
        let input = edid_by_filename(path);
        let bytes: [u8; 18] = input[0x48..=0x59].try_into().unwrap();

        let _t: i32 = 0b0000_0000_0010_1000;

        let got = parse(&bytes, &input).unwrap();

        let expected = RangeLimitsDesc::GtfSupported {
            limits: RangeLimits {
                min_v_rate_hz: 56,
                max_v_rate_hz: 75,
                min_h_rate_khz: 30,
                max_h_rate_khz: 81,
                offsets: Offsets {
                    vertical: VerticalOffset::Zero,
                    horizontal: HorizontalOffset::Zero,
                },
                max_pixel_clock_mhz: 140,
            },
        };

        assert_eq!(got, expected);
    }

    /**
    this device yields bad output - it has 0x00 for both horizontal rates.

    the library correctly detects this! :)

        ...
        01 01 01 01 01 01 3f 7f b0 a0 a0 20 34 70 30 20
        3a 00 04 ad 10 00 00 19 00 00 00 fd 00 30 3c 00
                        ^^^
        note: these values are nonconformant.
        ...

    */
    #[test]
    fn lgd0555_7d17e3014129() {
        logger();
        let path = "bad/linuxhw_edid_EDID_Digital_LG Display_LGD0555_7D17E3014129.input";
        let input = edid_by_filename(path);
        let bytes: [u8; 18] = input[0x48..0x5A].try_into().unwrap();

        let got = parse(&bytes, &input).unwrap();
        tracing::info!("{:#?}", got);

        let expected = RangeLimitsDesc::CvtSupported {
            limits: RangeLimits {
                min_v_rate_hz: 48,
                max_v_rate_hz: 60,
                min_h_rate_khz: 0,
                max_h_rate_khz: 0,
                offsets: Offsets {
                    vertical: VerticalOffset::Zero,
                    horizontal: HorizontalOffset::Zero,
                },
                max_pixel_clock_mhz: 330,
            },

            enhanced_px_clk: Decimal::from(328) + (Decimal::from(3) / Decimal::from(4)),
            cvt_version: 10,
            maximum_active_pxls_per_line: Some(160),
            supported_aspect_ratios: SupportedAspectRatios {
                _4x3: false,
                _16x9: false,
                _16x10: false,
                _5x4: true,
                _15x9: false,
            },
            preferred_aspect_ratio: PreferredAspectRatio::_4x3,
            supports_standard_cvt_blanking: false,
            supports_reduced_cvt_blanking: true,
            supports_h_shrink_scaling: false,
            supports_h_stretch_scaling: false,
            supports_v_shrink_scaling: false,
            supports_v_stretch_scaling: true,
            preferred_v_refresh_rate_hz: 20,
        };
        tracing::warn!("{:#?}", expected);

        assert_eq!(got, expected);
    }
}
