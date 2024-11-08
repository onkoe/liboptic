use bitvec::{field::BitField, view::BitView};

use crate::prelude::internal::*;

/// Finds the standard timings for this display.
#[tracing::instrument]
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
#[tracing::instrument]
fn one(bytes: &[u8]) -> STiming {
    // from [8, 2288] px
    let horizontal_addr_pixel_ct = hoz_addr_pixels(bytes[0]);

    // split the other byte into bits
    let bits: &bitvec::prelude::BitSlice<u8> = bytes[1].view_bits();

    // find aspect ratio
    let aspect_ratio = match (bits[7], bits[6]) {
        (false, false) => std_timings::StandardAspectRatio::_16_10,
        (false, true) => std_timings::StandardAspectRatio::_4_3,
        (true, false) => std_timings::StandardAspectRatio::_5_4,
        (true, true) => std_timings::StandardAspectRatio::_16_9,
    };

    let field_refresh_rate = bits[0..=5].load_be::<u8>() + 60;

    STiming {
        horizontal_addr_pixel_ct,
        aspect_ratio,
        field_refresh_rate,
    }
}

/// Finds the number of horizontal addressable pixels from the given value.
#[tracing::instrument]
fn hoz_addr_pixels(ct: u8) -> u16 {
    if ct == 0x00 {
        unreachable!("hoz addr pixels... reserved: do not use");
    }

    (ct as u16 + 31) * 8
}
