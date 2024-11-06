use winnow::{
    error::{ErrMode, ParserError as _},
    PResult,
};

mod header;
mod id;
pub(super) mod util;

pub fn parse(input: &mut &[u8]) -> PResult<super::Edid> {
    // check the length
    check_length(input)?;

    // make sure header's right
    header::parse(input)?;

    // grab vendor + product info
    let _vendor_product_info = id::parse(input)?;

    todo!()
}

fn check_length(input: &&[u8]) -> PResult<()> {
    let expected_len = 0x7F;
    let real_len = input.len();

    if real_len < expected_len {
        tracing::error!("The length is too short: (got: `{real_len}`, expected: `{expected_len}`)");
        return Err(ErrMode::from_error_kind(
            input,
            winnow::error::ErrorKind::Verify,
        ));
    }

    Ok(())
}
