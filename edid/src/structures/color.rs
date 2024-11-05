//! Color characteristics.

/// A representation of the CIE 1931 color space.
///
/// Indicates the colors a device can display.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ColorCharacteristics {
    red: ColorCoordinate,
    green: ColorCoordinate,
    blue: ColorCoordinate,
    white_point: ColorCoordinate,
}

/// A coordinate on the CIE 1931 color space graph, used to represent
/// which colors a device can display.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ColorCoordinate {
    pub x: f32,
    pub y: f32,
}
