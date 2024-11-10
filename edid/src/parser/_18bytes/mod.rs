use crate::{prelude::internal::*, structures::desc::DisplayDescriptor};
use descriptors::{range_limits, *};

mod descriptors;
mod preferred_tm;

#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8]) -> Result<EighteenByteDescriptors, EdidError> {
    let first = &input[0x36..=0x47];
    let second = &input[0x48..=0x59];
    let third = &input[0x5A..=0x6B];
    let fourth = &input[0x6C..=0x7D];

    // the first one is ALWAYS a preferred timing mode.
    let preferred_timing_mode = preferred_tm::parse(first.try_into()?);

    let blocks = [
        one(second.try_into()?, input)?,
        one(third.try_into()?, input)?,
        one(fourth.try_into()?, input)?,
    ];

    Ok(EighteenByteDescriptors {
        preferred_timing_mode,
        blocks,
    })
}

/// Parses the given eighteen-byte block.
#[tracing::instrument(skip_all)]
fn one(input: &[u8; 18], edid: &[u8]) -> Result<EighteenByteBlock, EdidError> {
    // if the first two bytes aren't both zero, it's a timing definition
    if input[0] != 0x00 && input[1] != 0x00 {
        return Ok(EighteenByteBlock::Timing(preferred_tm::parse(input)));
    }

    // otherwise, we're making a display descriptor.
    //
    // let's make sure it has the required reserved byte.
    {
        let reserved = input[2];
        if reserved != 0x00 {
            tracing::error!("EDID contained ambiguous descriptor! Malformed byte: `{reserved}`");
            return Err(EdidError::AmbiguousDescriptor {
                malformed_byte: reserved,
            });
        }
    }

    // okay! we know that it's some kind of descriptor.
    //
    // the specific kind we're making is indicated at byte 3. let's see what
    // that is and call the appropriate parser
    let kind_byte = input[3];
    let desc = match kind_byte {
        // string friends
        0xFF => DisplayDescriptor::ProductSerial(_13_byte_string::parse(input)),
        0xFE => DisplayDescriptor::DataString(_13_byte_string::parse(input)),
        0xFC => DisplayDescriptor::ProductName(_13_byte_string::parse(input)),

        // others
        0xFD => DisplayDescriptor::DisplayRangeLimits(range_limits::parse(input, edid)?),
        0xFB => descriptors::color_point::parse(input),
        0xFA => descriptors::more_std_timings::parse(input)?,

        // errors
        0x11..=0xF6 => return Err(EdidError::DescriptorUsedReservedKind { kind_byte }),
        ty => todo!("this descriptor type (`{ty:x}`) is unimplemented!"),
    };

    Ok(EighteenByteBlock::Display(desc))
}
