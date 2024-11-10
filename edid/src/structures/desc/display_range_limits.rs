use rust_decimal::Decimal;

/// Tagged with 0xFD.
///
/// An optional* tag with info about the range limits and maximum pixel clock
/// frequency for a display.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum RangeLimitsDesc {
    /// Range limits only. No additional timing information is provided.
    ///
    /// Indicates that Default GTF, GTF Secondary Curve, and CVT are all
    /// unsupported.
    LimitsOnly {
        limits: RangeLimits,

        /// When true, this display will present an image with any valid video
        /// mode within tge range limits. Note that the displayed image may not
        /// be properly sized or centered.
        ///
        /// If it's false, though, it will only present an image when a valid
        /// video timing configuration is applied.
        ///
        /// These include both those shown in the base EDID as well as those in
        /// any extension block(s).
        flexible: bool,
    },

    /// Indicates that Default GTF is supported, but does not provide data.
    GtfSupported { limits: RangeLimits },

    /// Indicates that GTF Secondary Curve is supported. Provides data!
    GtfSecondaryCurveSupported {
        /// The range limits.
        limits: RangeLimits,

        /// = (hfreq / 2)
        start_break_freq: u16,
        /// = (2c)
        c2: u8,
        /// real m.
        m: u16,
        /// real k
        k: u8,
        /// = (2j)
        j2: u8,
    },

    /// Indicates that CVT is supported and provides data for it!
    CvtSupported {
        /// The range limits.
        limits: RangeLimits,

        /// The pixel clock from the range limits, with enhanced accuracy (to 0.25 MHz).
        enhanced_px_clk: Decimal,

        /// The version of CVT implemented. For example, 11_u8 is v1.1.
        cvt_version: u8,

        /// Optional since there can be "no limit".
        maximum_active_pxls_per_line: Option<u16>,
        supported_aspect_ratios: SupportedAspectRatios,
        preferred_aspect_ratio: PreferredAspectRatio,

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

impl RangeLimitsDesc {
    /// Gets the range limits for any variant of this enum.
    pub fn limits(&self) -> RangeLimits {
        match self {
            Self::LimitsOnly { limits, .. } => limits.clone(),
            Self::GtfSupported { limits } => limits.clone(),
            Self::GtfSecondaryCurveSupported { limits, .. } => limits.clone(),
            Self::CvtSupported { limits, .. } => limits.clone(),
        }
    }
}

/// Includes info about the min + max values of the vertical/horizontal
/// scanning rate and the maximum supported pixel clock frequency.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct RangeLimits {
    /// Minimum Vertical Rate in Hz.
    pub min_v_rate_hz: u16,
    /// Maximum Vertical Rate in Hz.
    pub max_v_rate_hz: u16,

    /// Minimum Horizontal Rate in kHz.
    pub min_h_rate_khz: u16,
    /// Maximum Horizontal Rate in kHz.
    pub max_h_rate_khz: u16,

    /// Any offsets augmenting the above fields.
    pub offsets: Offsets,

    /// The maximum pixel clock in MHz. Note that this is in multiples of ten.
    pub max_pixel_clock_mhz: u16,
}

/// Info about any potential range limit offsets.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Offsets {
    pub vertical: VerticalOffset,
    pub horizontal: HorizontalOffset,
}

impl Offsets {
    /// Returns true if there is any offset, in any direction.
    ///
    /// In other words, returns false when both directions are Zero.
    pub fn is_offset(&self) -> bool {
        self.horizontal != HorizontalOffset::Zero && self.vertical != VerticalOffset::Zero
    }
}

/// Vertical range limit offset.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[expect(non_camel_case_types, reason = "enhances readability a lot")]
pub enum VerticalOffset {
    Zero,
    Max255Hz_MinNotOffset,
    Max255Hz_Min255Hz,
}

impl VerticalOffset {
    /// Checks if there's a maximum offset.
    pub fn has_max(&self) -> bool {
        *self == Self::Max255Hz_Min255Hz || *self == Self::Max255Hz_MinNotOffset
    }

    /// Checks if there's a minimum offset.
    pub fn has_min(&self) -> bool {
        *self == Self::Max255Hz_Min255Hz
    }
}

/// Horizontal range limit offset.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[expect(non_camel_case_types, reason = "enhances readability a lot")]
pub enum HorizontalOffset {
    Zero,
    Max255kHz_MinNotOffset,
    Max255kHz_Min255kHz,
}

impl HorizontalOffset {
    /// Checks if there's a maximum offset.
    pub fn has_max(&self) -> bool {
        *self == Self::Max255kHz_Min255kHz || *self == Self::Max255kHz_MinNotOffset
    }

    /// Checks if there's a minimum offset.
    pub fn has_min(&self) -> bool {
        *self == Self::Max255kHz_Min255kHz
    }
}

/// All of the supported aspect ratios.
///
/// These are defined in the EDID standard and aren't arbitrary.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
#[non_exhaustive]
#[repr(C)]
pub struct SupportedAspectRatios {
    pub _4x3: bool,
    pub _16x9: bool,
    pub _16x10: bool,
    pub _5x4: bool,
    pub _15x9: bool,
}

/// A supported aspect ratio for a display.
///
/// These are defined in the EDID standard and aren't arbitrary.
#[derive(Clone, Debug, PartialEq, PartialOrd, Hash)]
#[non_exhaustive]
#[repr(C)]
pub enum PreferredAspectRatio {
    _4x3,
    _16x9,
    _16x10,
    _5x4,
    _15x9,
}
