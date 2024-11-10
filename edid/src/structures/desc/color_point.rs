use rust_decimal::Decimal;

use super::color::ColorCoordinate;

/// A white point coordinate on the CIE 1931 color space graph.
///
/// These are used in the 18 byte descriptors when additional colors are given.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WhitePoint {
    pub index_number: u8,
    pub coord: ColorCoordinate,

    /// If None, then the gamma is defined in an extension block.
    pub gamma: Option<Decimal>,
}
