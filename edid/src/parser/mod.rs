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

    // grab vendor + product info
    let _vendor_product_info = id::parse(input)?;

    // edid standard version + revision
    let _rnv = version::parse(input)?;

    // construct the type
    // (todo)

    // finalized checks
    if input[0x18] == 1 {
        todo!(
            "Ensure Display Range Limits are included as a block in the base \
        EDID. See pg. 40, table 3.26, note 1."
        )
    }

    todo!()
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
