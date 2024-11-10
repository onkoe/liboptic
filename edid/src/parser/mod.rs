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

pub fn parse(input: &[u8]) -> Result<Edid, EdidError> {
    // check the length
    check_length(input)?;

    // make sure header's right
    header::parse(input)?;

    // construct the type
    let edid = Edid {
        vendor_product_info: id::parse(input)?,
        version: version::parse(input)?,
        basic_display_info: basic_info::parse(input)?,
        color_characteristics: color::parse(input),
        established_timings: est_timings::parse(input),
        standard_timings: std_timings::parse(input),
        eighteen_byte_data_blocks: _18bytes::parse(input)?,
        extension_info: input[0x7E],
        checksum: checksum(input),
    };

    // finalized checks
    {
        // when the display is cont. freq., we check if the display range limits
        // descriptor is given
        let has_range_desc = !edid.eighteen_byte_data_blocks.blocks.iter().any(|b| {
            matches!(
                b,
                EighteenByteBlock::Display(DisplayDescriptor::DisplayRangeLimits(_))
            )
        });
        if input[0x18] == 1 && has_range_desc {
            tracing::warn!(
                "This EDID is for a continuous display, but it didn't not contain \
            the required Display Range Limits and Timing Descriptor."
            );
        }
    }

    Ok(edid)
}

fn check_length(input: &[u8]) -> Result<(), EdidError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dell_s2417dg_edid() {
        logger();
        let name = "dell_s2417dg.raw.input";
        let input = raw_edid_by_filename(name);

        let _got = super::parse(&input).unwrap();

        // todo: remake that entire damn edid
    }

    /// try some edid v1.3s to ensure a least a lil compatability
    #[test]
    fn edid_v1_3() {
        parse(&edid_by_filename(
            "linuxhw_edid_Digital_MSI_MSIAF82_4B2991D4299A.input",
        ))
        .unwrap();

        parse(&edid_by_filename(
            "linuxhw_edid_Analog_AOC_AOC0320_455954E7CA14.input",
        ))
        .unwrap();
    }
}
