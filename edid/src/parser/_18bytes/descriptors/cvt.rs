//! Coordinated Video Timings (CVT) descriptor (0xf8)
//!
//! Once again, this seems to be unused by vendors. Zero implementors out of
//! the 100k hw-probe display submissions. So no tests here. Please PR if you
//! have a real-world sample of this data.

use bitvec::{field::BitField, order::Lsb0, view::BitView};

use crate::{
    prelude::internal::*,
    structures::desc::cvt_3_byte_timing::{
        CvtAspectRatio, CvtPreferredVerticalRate, SupportedVRates, TimingCodeDesc,
    },
};

/// Parses out a CVT descriptor from the given input.
#[tracing::instrument]
pub(crate) fn parse(input: &[u8; 18]) -> Result<DisplayDescriptor, EdidError> {
    let version_number = input[5];

    // warn if they're using a reserved value (which is anything but 0x01)
    if version_number != 0x01 {
        tracing::warn!(
            "Coordinated video timings version used a reserved value: `{version_number}`"
        );
    }

    let first = one(&input[6..9].try_into()?).ok_or_else(|| {
        tracing::error!("The first CVT code must be defined, but this descriptor didn't do that.");
        EdidError::DescriptorNoFirstCvt
    })?;

    Ok(DisplayDescriptor::Cvt3ByteTimingCodes {
        version_number,
        first,
        second: one(&input[9..12].try_into()?),
        third: one(&input[12..15].try_into()?),
        last: one(&input[15..18].try_into()?),
    })
}

/// Parses one of the four TimingCodeDescs fields.
#[tracing::instrument]
fn one(input: &[u8; 3]) -> Option<TimingCodeDesc> {
    // if the values are all zero, it's unused
    if input == &[0x00, 0x00, 0x00] {
        tracing::info!("Given empty CVT code. Returning none...");
        return None;
    }

    let bits1 = input[1].view_bits::<Lsb0>();
    let addressable_lines = {
        // grab the larger four bits.
        let upper = bits1[0x04..=0x07].load::<u8>();

        // and the byte
        let c: u16 = bytemuck::must_cast([input[0], upper]);
        c
    };

    // grab the ar
    let aspect_ratio = match [bits1[3], bits1[2]] {
        [false, false] => CvtAspectRatio::_4_3,
        [false, true] => CvtAspectRatio::_16_9,
        [true, false] => CvtAspectRatio::_16_10,
        [true, true] => CvtAspectRatio::_15_9,
    };

    // check if reserved bits are being used
    let b1_reserved = {
        let b = &input[1].view_bits::<Lsb0>();
        [b[0], b[1]]
    };
    if b1_reserved != [false, false] {
        tracing::warn!("Some of the reserved bits on middle byte are set: {b1_reserved:?}");
    }

    // another check
    let bits2 = input[2].view_bits::<Lsb0>();
    if bits2[7] {
        tracing::warn!("Reserved bit on last CVT code byte is set to 1.");
    }

    // pref refresh rate
    let preferred_vertical_rate = match [bits2[6], bits2[5]] {
        [false, false] => CvtPreferredVerticalRate::_50Hz,
        [false, true] => CvtPreferredVerticalRate::_50Hz,
        [true, false] => CvtPreferredVerticalRate::_75Hz,
        [true, true] => CvtPreferredVerticalRate::_85Hz,
    };

    // supported refresh
    let supported_vertical_rates = SupportedVRates {
        _50_hz_standard: bits2[4],
        _60_hz_standard: bits2[3],
        _75_hz_standard: bits2[2],
        _85_hz_standard: bits2[1],
        _60_hz_reduced: bits2[0],
    };

    Some(TimingCodeDesc {
        addressable_lines,
        aspect_ratio,
        preferred_vertical_rate,
        supported_vertical_rates,
    })
}
