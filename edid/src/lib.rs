//! # `liboptic_edid`
//!
//! A library crate to parse EDID information.
//!
//! Currently, this uses EDID v1.4 and does not account for any
//! incompatbilities with earlier versions.

#![no_std]

mod parser;
mod prelude;
pub mod structures; // TODO: re-export with prelude instead

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
    /// A static identifier for EDID structures. Should always be here.
    header: [u8; 8],

    /// Info about the product vendor.
    vendor_product_info: id::VendorProductId,

    /// The version + revision that this EDID was created for.
    version: version::EdidVersion,

    /// Basic info about the display, indicating important things like the
    /// its input and screen size.
    basic_display_info: basic_info::BasicDisplayInfo,

    /// Info about this display's placements in the CIE 1931 color space.
    color_characteristics: color::ColorCharacteristics,

    /// Various hardcoded timing booleans.
    esablished_timings: est_timings::EstablishedTimings,

    /// Dynamic timings.
    standard_timings: std_timings::StandardTimings,

    /// Four 18-byte data blocks with info about the display and/or its
    /// timings.
    eighteen_byte_data_blocks: _18bytes::EighteenByteDescriptors,

    /// Info about the E-EDID extensions this EDID carries behind it.
    extension_info: extension::ExtensionInfo,
}

impl Edid {
    pub fn new(edid_data: &mut &[u8]) -> PResult<Self> {
        parser::parse(edid_data)
    }
}
