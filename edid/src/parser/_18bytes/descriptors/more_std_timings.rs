use crate::prelude::internal::*;

use crate::parser::std_timings::one;

/// Parses... six more standard timings. In case your display is using all
/// those.
#[tracing::instrument(skip(input))]
pub(crate) fn parse(input: &[u8; 18]) -> Result<DisplayDescriptor, EdidError> {
    // error if weird header value
    let header = &input[0..5];
    if header != [0x00, 0x00, 0x00, 0xFA, 0x00] {
        tracing::error!("Given descriptor data had a malformed header: {header:x?}");
        return Err(EdidError::DescriptorUnexpectedHeader(header.try_into()?));
    }

    // check if the last value matches the expected one. i'll let this one fly
    if let Some(last) = input.last() {
        if *last != 0x0A {
            tracing::warn!(
                "Standard timings display descriptor used reserved byte 17. (value: {last}"
            );
        }
    }

    Ok(DisplayDescriptor::StandardTimingIdentifications {
        _9: one(&input[5..7]),
        _10: one(&input[7..9]),
        _11: one(&input[9..11]),
        _12: one(&input[11..13]),
        _13: one(&input[13..15]),
        _14: one(&input[15..17]),
    })
}
