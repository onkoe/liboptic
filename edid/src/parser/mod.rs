mod _18bytes;
mod basic_info;
mod color;
mod est_timings;
mod header;
mod id;
mod std_timings;
pub(super) mod util;
mod version;

use crate::prelude::internal::*;

pub fn parse(input: &mut &[u8]) -> Result<Edid, EdidError> {
    // check the length
    check_length(input)?;

    // make sure header's right
    header::parse(input)?;

    // construct the type
    let edid = Edid {
        vendor_product_info: id::parse(input)?,
        version: version::parse(input)?,
        basic_display_info: basic_info::parse(input),
        color_characteristics: color::parse(input),
        established_timings: est_timings::parse(input),
        standard_timings: std_timings::parse(input),
        eighteen_byte_data_blocks: _18bytes::parse(input)?,
        extension_info: input[0x7E],
        checksum: checksum(input),
    };

    // finalized checks
    if input[0x18] == 1 {
        todo!(
            "Ensure Display Range Limits are included as a block in the base \
        EDID. See pg. 40, table 3.26, note 1."
        )
    }

    Ok(edid)
}

fn check_length(input: &&[u8]) -> Result<(), EdidError> {
    let expected_len = 0x7F;
    let real_len = input.len();

    if real_len < expected_len {
        tracing::error!("The length is too short: (got: `{real_len}`, expected: `{expected_len}`)");

        return Err(EdidError::TooShort {
            got: real_len as u8,
            expected: expected_len as u8,
        });
    }

    Ok(())
}

/// Returns the checksum byte to the user.
#[tracing::instrument]
fn checksum(input: &[u8]) -> u8 {
    let sum = |bytes: &[u8]| bytes.iter().map(|b| *b as u32).sum::<u32>();

    // warn the user if the checksum is wrong
    let all = sum(&input[..=0x7F]) % 256;
    if all != 0x00 {
        tracing::error!(
            "The given EDID failed its checksum. It will still be included in the type."
        );
    }

    input[0x7F]
}
