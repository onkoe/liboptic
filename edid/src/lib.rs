//! # `liboptic_edid`
//!
//! A library crate to parse EDID information.
//!
//! Currently, this uses EDID v1.4 and does not account for any
//! incompatbilities with earlier versions.
//!
//! ## Usage
//!
//! The only significant type in the library is [`Edid`]. Call [`Edid::new()`]
//! with your EDID in bytes to get it parsed!

#![no_std]

pub mod error;
mod parser;
mod prelude;
pub mod structures;

use crate::prelude::internal::*;

/// The latest version of the EDID standard that this library includes support
/// for.
pub const LATEST_SUPPORTED_VERSION: u8 = 0x01;

/// The latest revision of the standard this library supports.
pub const LATEST_SUPPORTED_REVISION: u8 = 0x04;

/// The base EDID structure.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Edid {
    /// Info about the product vendor.
    pub vendor_product_info: id::VendorProductId,

    /// The version + revision that this EDID was created for.
    pub version: version::EdidVersion,

    /// Basic info about the display, indicating important things like the
    /// its input and screen size.
    pub basic_display_info: basic_info::BasicDisplayInfo,

    /// Info about this display's placements in the CIE 1931 color space.
    pub color_characteristics: color::ColorCharacteristics,

    /// Various hardcoded timing booleans.
    pub established_timings: est_timings::EstablishedTimings,

    /// Dynamic timings.
    pub standard_timings: std_timings::StandardTimings,

    /// Four 18-byte data blocks with info about the display and/or its
    /// timings.
    pub eighteen_byte_data_blocks: _18bytes::EighteenByteDescriptors,

    /// The number of extension blocks (including optional block map/s) that
    /// follow the base EDID.
    ///
    /// Limited up to 255, as indicated by the type.
    pub extension_info: u8,

    /// Some value that makes the EDID's checksum be 0x00.
    pub checksum: u8,
}

impl Edid {
    /// Creates a new `Edid` from the raw bytes of the given `edid_data`.
    ///
    /// Please ensure that the method you're using to read the EDID does not
    /// return a String. Instead, it should return a byte slice. For example,
    /// `std::fs::read` works great for this!
    ///
    /// ```edition2021
    /// use liboptic_edid::Edid;
    ///
    /// // grab the edid file from disk
    /// let data = std::fs::read("tests/assets/dell_s2417dg.raw.input")?;
    ///
    /// // and load it into the parser
    /// let parsed_edid = Edid::new(&data)?;
    /// assert_eq!(parsed_edid.checksum, 0x51);
    /// #
    /// # Ok::<(), Box<dyn core::error::Error>>(())
    /// ```
    pub fn new(edid_data: &[u8]) -> Result<Self, EdidError> {
        parser::parse(edid_data)
    }
}
