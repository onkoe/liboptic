use bitvec::{field::BitField, view::BitView};

use crate::prelude::internal::*;

/// Finds the standard timings for this display.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8]) -> StandardTimings {
    // yeah im not using unsafe to avoid the array here. hardcoding it is :D
    StandardTimings {
        st1: one(&input[0x26..=0x27]),
        st2: one(&input[0x28..=0x29]),
        st3: one(&input[0x2a..=0x2b]),
        st4: one(&input[0x2c..=0x2d]),
        st5: one(&input[0x2e..=0x2f]),
        st6: one(&input[0x30..=0x31]),
        st7: one(&input[0x32..=0x33]),
        st8: one(&input[0x34..=0x35]),
    }
}

/// Checks the info of the given standard timing.
///
/// Make sure the given bytes are correctly aligned.
///
/// While these aren't noted to be optional in the standard, vendors tend to
/// treat them as if they were.
#[tracing::instrument]
pub(crate) fn one(bytes: &[u8]) -> Option<STiming> {
    // if both bytes are 1, assume unused
    if (bytes[0] == 0x01 && bytes[1] == 0x01) {
        return None;
    }

    // if the first byte is 1, also assume unused. warn about its usage.
    if (bytes[0] == 0x01 && bytes[1] == 0x00) {
        tracing::warn!(
            "Standard timing used an [0x01, 0x00] to show that the timing \
        was unused. This is against the standard. \
        Consider using [0x01, 0x01] instead."
        );
        return None;
    }

    // from [256, 2288] px
    let horizontal_addr_pixel_ct = hoz_addr_pixels(bytes[0])?;

    // split the other byte into bits
    let bits: &bitvec::prelude::BitSlice<u8> = bytes[1].view_bits();

    // find aspect ratio
    tracing::debug!("aspect ratio: {:#?}", (bits[7], bits[6]));
    let aspect_ratio = match (bits[7], bits[6]) {
        (false, false) => StandardAspectRatio::_16_10,
        (false, true) => StandardAspectRatio::_4_3,
        (true, false) => StandardAspectRatio::_5_4,
        (true, true) => StandardAspectRatio::_16_9,
    };

    let field_refresh_rate = bits[0..=5].load_be::<u8>() + 60;

    Some(STiming {
        horizontal_addr_pixel_ct,
        aspect_ratio,
        field_refresh_rate,
    })
}

/// Finds the number of horizontal addressable pixels from the given value.
#[tracing::instrument]
fn hoz_addr_pixels(ct: u8) -> Option<u16> {
    if ct == 0x00 {
        tracing::warn!(
            "Standard timing used 0x00 pixel count, which isn't permitted. Returning None."
        );
        return None;
    }

    Some((ct as u16 + 31) * 8)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _2c47316eff13_std_timings() {
        logger();
        let path = "linuxhw_edid_EDID_Digital_Samsung_SAM02E3_2C47316EFF13.input";
        let input = crate::prelude::internal::edid_by_filename(path);

        let got = super::parse(&input);
        tracing::info!("got: {got:#?}");

        let expected = StandardTimings {
            st1: Some(STiming {
                horizontal_addr_pixel_ct: 1440,
                aspect_ratio: StandardAspectRatio::_16_10,
                field_refresh_rate: 60,
            }),
            st2: Some(STiming {
                horizontal_addr_pixel_ct: 1440,
                aspect_ratio: StandardAspectRatio::_16_10,
                field_refresh_rate: 75,
            }),
            st3: Some(STiming {
                horizontal_addr_pixel_ct: 1280,
                aspect_ratio: StandardAspectRatio::_5_4,
                field_refresh_rate: 60,
            }),
            st4: Some(STiming {
                horizontal_addr_pixel_ct: 1280,
                aspect_ratio: StandardAspectRatio::_4_3,
                field_refresh_rate: 60,
            }),
            st5: Some(STiming {
                horizontal_addr_pixel_ct: 1152,
                aspect_ratio: StandardAspectRatio::_4_3,
                field_refresh_rate: 75,
            }),
            st6: None,
            st7: None,
            st8: None,
        };
        tracing::warn!("expected: {expected:#?}");

        assert_eq!(got, expected);
    }
}
