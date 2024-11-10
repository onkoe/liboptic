//! Color management descriptor (tag 0xf9 or "f9h")
//!
//! Note that this is called DCM because of the VESA DCM standard, which
//! stands for "Display Color Management".
//!
//! Also, please note that this module (currently) does not implement DCM,
//! but instead just returns the raw values.

use crate::prelude::internal::*;

/// Parses out a DCM descriptor from the given bytes.
#[tracing::instrument]
pub(crate) fn parse(input: &[u8; 18]) -> DisplayDescriptor {
    let version_number = input[5];

    if version_number != 0x03 {
        tracing::warn!(
            "DCM version number was 0x00, but values except for 0x03 \
        are reserved."
        );
    }

    DisplayDescriptor::DcmData {
        version_number,
        red_a3: bytemuck::must_cast([input[6], input[7]]),
        red_a2: bytemuck::must_cast([input[8], input[9]]),
        green_a3: bytemuck::must_cast([input[10], input[11]]),
        green_a2: bytemuck::must_cast([input[12], input[13]]),
        blue_a3: bytemuck::must_cast([input[14], input[15]]),
        blue_a2: bytemuck::must_cast([input[16], input[17]]),
    }
}
