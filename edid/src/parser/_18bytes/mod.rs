use crate::{prelude::internal::*, structures::desc::DisplayDescriptor};
use descriptors::{range_limits, *};

mod descriptors;
mod preferred_tm;

/// Parses out the four 18-byte descriptors from the user's provided EDID input.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8]) -> Result<EighteenByteDescriptors, EdidError> {
    let first = &input[0x36..=0x47];
    let second = &input[0x48..=0x59];
    let third = &input[0x5A..=0x6B];
    let fourth = &input[0x6C..=0x7D];

    // in EDID v1.3 and v1.4, the first 18-byte block will have the display's
    // preferred timings.
    //
    // however, this isn't always the case on earlier versions, so the name may
    // not match the type.
    let preferred_timing_mode = one(first.try_into()?, input)?;
    if matches!(preferred_timing_mode, EighteenByteBlock::Display(_)) {
        tracing::warn!(
            "The first 18-byte block was not a preferred timing descriptor. \
        In EDID v1.3 and v1.4, this is not conformant with the standard."
        );
    }

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
        return Ok(EighteenByteBlock::Timing(preferred_tm::parse(input)?));
    }

    // otherwise, we're making a display descriptor.
    {
        // let's check the reserved bytes
        let header = &input[0..5];
        if !matches!(header, [0x00, 0x00, 0x00, _, 0x00]) {
            tracing::error!("Given descriptor data had a malformed header: {header:x?}");
            return Err(EdidError::DescriptorUnexpectedHeader(header.try_into()?));
        }
    }

    // okay! we've confirmed that it's a valid descriptor.
    //
    // the specific kind we're making is indicated at byte 3. let's see what
    // that is and call the appropriate parser
    let kind_byte = input[3];
    let desc = match kind_byte {
        // string friends
        0xFF => DisplayDescriptor::ProductSerial(_13_byte_string::parse(input)?),
        0xFE => DisplayDescriptor::DataString(_13_byte_string::parse(input)?),
        0xFC => DisplayDescriptor::ProductName(_13_byte_string::parse(input)?),

        // others
        0xFD => DisplayDescriptor::DisplayRangeLimits(range_limits::parse(input, edid)?),
        0xFB => descriptors::color_point::parse(input),
        0xFA => descriptors::more_std_timings::parse(input)?,
        0xF9 => descriptors::dcm::parse(input),
        0xF8 => descriptors::cvt::parse(input)?, // this one isn't used in ANY of 100k samples lol
        0xF7 => descriptors::iii::parse(input)?,
        0x10 => {
            // check if it contains data (it shouldn't)
            if input[5..=17].iter().map(|i| *i as u16).sum::<u16>() != 0 {
                tracing::warn!("The EDID supplied a dummy 18-byte descriptor, but it contained data! (data: {input:?})");
            }

            DisplayDescriptor::DummyDescriptor
        }

        // manufacturer
        m if (0x00..=0x0F).contains(&m) => {
            tracing::debug!("Got a manufacturer descriptor. (tag: `{m:x}`)");
            DisplayDescriptor::Manufacturer { data: *input }
        }

        // errors
        tag if (0x11..=0xF6).contains(&tag) => {
            tracing::error!("EDID supplied an 18-byte descriptor that used a reserved tag. (tag: ");
            return Err(EdidError::DescriptorUsedReservedKind { kind_byte });
        }
        ty => todo!("this descriptor type (`{ty:x}`) is unimplemented!"),
    };

    Ok(EighteenByteBlock::Display(desc))
}
