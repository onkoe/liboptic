use crate::prelude::internal::*;

mod preferred_tm;

#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8]) -> Result<EighteenByteDescriptors, EdidError> {
    let first = &input[0x36..=0x47];
    let second = &input[0x48..=0x59];
    let third = &input[0x5A..=0x6B];
    let fourth = &input[0x6C..=0x7D];

    // the first one is ALWAYS a preferred timing mode.
    let first_parsed = preferred_tm::parse(first.try_into()?);

    todo!()
}
