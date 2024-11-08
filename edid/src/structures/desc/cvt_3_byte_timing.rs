#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TimingCodeDesc {
    /// really a "u12"
    pub addressable_lines: u16,

    pub aspect_ratio: CvtAspectRatio,
    pub preferred_vertical_rate: CvtPreferredVerticalRate,
    pub supported_vertical_rates: SupportedVRates,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum CvtAspectRatio {
    _16x10,
    _4x3,
    _5x4,
    _16x9,
}

/// The referred vertical rate.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum CvtPreferredVerticalRate {
    _50Hz,
    _60Hz,
    _75Hz,
    _85Hz,
}

/// The supported vertical rates and their blanking styles.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SupportedVRates {
    pub _50_hz_standard: bool,
    pub _60_hz_standard: bool,
    pub _75_hz_standard: bool,
    pub _85_hz_standard: bool,
    pub _60_hz_reduced: bool,
}
