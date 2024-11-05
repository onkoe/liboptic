/// Tagged with 0xFD.
///
/// An optional tag with info about the range limits and maximum pixel clock
/// frequency for a display.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct DisplayRangeLimits {
    /// Minimum Vertical Rate in Hz.
    pub min_v_rate: u16,
    /// Maximum Vertical Rate in Hz.
    pub max_v_rate: u16,

    /// Minimum Horizontal Rate in kHz.
    pub min_h_rate: u16,
    /// Maximum Horizontal Rate in kHz.
    pub max_h_rate: u16,

    /// The maxmimum pixel clock in MHz. Note that this is in multiples of ten.
    //
    // IMPLEMENTATION NOTE: mult by ten!
    pub max_pixel_clock: u32,

    pub timing_formula: TimingFormula,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TimingFormula {
    // gtf or gtfo
    GtfSecondaryCurve {
        /// = (hfreq / 2)
        start_break_freq: u16,
        /// = (2c)
        c: u8,
        /// real m. stored as lsb first.
        m: u16,
        /// real k
        k: u8,
        /// = (2j)
        j: u8,
    },

    // cvt or... uhh
    Cvt {
        cvt_major_version: u8,
        cvt_minor_version: u8,

        /// Optional since there can be "no limit".
        maximum_active_pxls_per_line: Option<u32>,
        supported_aspect_ratios: AspectRatio,
        preferred_aspect_ratios: AspectRatio,

        supports_standard_cvt_blanking: bool,
        supports_reduced_cvt_blanking: bool,

        supports_h_shrink_scaling: bool,
        supports_h_stretch_scaling: bool,
        supports_v_shrink_scaling: bool,
        supports_v_stretch_scaling: bool,

        /// The "preferred" vertical refresh rate in Hz.
        preferred_v_refresh_rate_hz: u8,
    },
}

/// A supported aspect ratio for a display.
///
/// These are defined in the EDID standard and aren't arbitrary.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
#[non_exhaustive]
#[repr(C)]
pub enum AspectRatio {
    _4x3,
    _16x9,
    _16x10,
    _5x4,
    _15x9,
}
