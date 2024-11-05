//! # `liboptic_edid`
//!
//! A library crate to parse EDID information.
//!
//! Currently, this uses EDID v1.4 and does not account for any
//! incompatbilities with earlier versions.

#![no_std]

use structures::{_18bytes, basic_info, color, est_timings, extension, id, std_timings, version};

// TODO: re-export with prelude instead
pub mod structures;

pub const EDID_HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

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
