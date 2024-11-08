extern crate alloc;
use alloc::format;
use arrayvec::ArrayString;

use core::{array::TryFromSliceError, error::Error};
use pisserror::Error;

/// An error that occurred while parsing EDID.
#[derive(Clone, Debug, Error)]
pub enum EdidError {
    #[error("The given EDID data isn't long enough.")]
    TooShort { got: u8, expected: u8 },

    // header
    #[error("The EDID header is too short.")]
    HeaderTooShort { real_len: u8, expected_len: u8 },
    #[error("The given EDID does not contain the expected EDID header.")]
    NoHeader,

    // id
    #[error("Couldn't find the header's manufacturer from its PNP ID.")]
    NoMatchingManufacturer(ArrayString<3>),
    #[error("Failed to find suitable Rust character for given value: {_0}")]
    CharOutOfBounds(u8),
    #[error("The parser was incorrectly given a `0x00` code.")]
    IdNoZeroesAllowed,

    // 18 byte blocks
    #[error(
        "The EDID has a weird 18-byte descriptor. It's not a timing, but didn't \
    include the reserved byte. It has byte `{malformed_byte}` instead!"
    )]
    AmbiguousDescriptor { malformed_byte: u8 },
    #[error("This EDID contained a reserved descriptor kind byte: `{kind_byte}`.")]
    DescriptorUsedReservedKind { kind_byte: u8 },

    // misc (logic errors that were noticed in other crates)
    #[error("An ArrayString had an overflow. Please report this alongside any logs.")]
    ArrayStringError,
    #[error("Failed to convert slice into array. (err: {_0})")]
    TryFromSlice(#[from] TryFromSliceError),
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Error)]
pub enum IdError {}
