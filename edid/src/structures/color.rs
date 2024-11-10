//! Color characteristics.

use rust_decimal::Decimal;

/// A representation of the CIE 1931 color space.
///
/// Indicates the colors a device can display.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ColorCharacteristics {
    pub red: ColorCoordinate,
    pub green: ColorCoordinate,
    pub blue: ColorCoordinate,
    pub white_point: ColorCoordinate,
}

/// A coordinate on the CIE 1931 color space graph, used to represent
/// which colors a device can display.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ColorCoordinate {
    pub x: Decimal,
    pub y: Decimal,
}

impl ColorCoordinate {
    /// Creates a new color coordinate on the chart.
    pub fn new(x: Decimal, y: Decimal) -> Self {
        Self { x, y }
    }
}
