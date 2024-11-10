extern crate alloc;
use alloc::format;
use arrayvec::ArrayString;

use core::{array::TryFromSliceError, error::Error, fmt::Debug};
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
    #[error("Range limits descriptor found reserved bits set: `{input:x?}`.")]
    DescriptorRangeLimitsUsedReservedBits {
        /// the input byte.
        input: u8,
    },
    #[error("Range limits descriptor contained a reserved video timing support flag: `{flag}`.")]
    DescriptorRangeLimitsUsedReservedVTSFlag { flag: u8 },
    #[error("Range limits descriptor (CVT) contained reserved values.")]
    DescriptorRangeLimitsCvtReservedBits,
    #[error("Descriptor used an unexpected value within the first five bytes. (bytes: {_0:x?})")]
    DescriptorUnexpectedHeader([u8; 5]),
    #[error("This EDID didn't provide the first CVT in its CVT descriptor.")]
    DescriptorNoFirstCvt,

    // misc (logic errors that were noticed in other crates)
    #[error("An ArrayString had an overflow. Please report this alongside any logs.")]
    ArrayStringError,
    #[error("Failed to convert slice into array. (err: {_0})")]
    TryFromSlice(#[from] TryFromSliceError),
    #[error(
        "Couldn't represent given number as binary-coded decimal. Please \
    report this alongside any logs."
    )]
    BcdError,
}
