use winnow::PResult;

mod header;
pub(super) mod util;

pub fn parse(input: &mut &[u8]) -> PResult<super::Edid> {
    // make sure header's right
    header::parse(input)?;

    todo!()
}
