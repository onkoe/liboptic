/// A collection of "standard" timings for a device.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct StandardTimings {
    pub st1: STiming,
    pub st2: STiming,
    pub st3: STiming,
    pub st4: STiming,
    pub st5: STiming,
    pub st6: STiming,
    pub st7: STiming,
    pub st8: STiming,
}

/// One standard timing.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct STiming {
    /// The horizontal active pixels.
    //
    // IMPLEMENTATION NOTE: (raw + 31) * 8
    pub horizontal_addr_pixel_ct: u16,
    pub aspect_ratio: StandardAspectRatio,
    /// The refresh rate in Hz.
    ///
    /// IMPLEMENTATION NOTE: raw + 60
    pub field_refresh_rate: u8,
}

/// The aspect ratio of a standard timing.
///
/// Limited to these values by the standard. See the spec for more info.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum StandardAspectRatio {
    _16_10,
    _4_3,
    _5_4,
    _16_9,
}
