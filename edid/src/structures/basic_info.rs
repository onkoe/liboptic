//! Basic display info.

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct BasicDisplayInfo {
    input_definition: vsi::VideoSignalInterface,

    /// the screen size or aspect ratio. the display may only report
    /// its aspect ratio or nothing at all!
    screen_size_or_aspect_ratio: SizeOrRatio,

    /// Whether the device reports a gamma value in an extension block.
    reports_gamma: bool,

    /// Info about the display's support for various misc. features.
    feature_support: feature_support::FeatureSupport,
}

pub mod vsi {
    /// Either an analog or digital VSI.
    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum VideoSignalInterface {
        // IMPLEMENTATION NOTE:
        //
        // if the first _BIT_ of the input is `0`, use Analog.
        // else if `1`, it's Digital.
        //
        Analog {
            signal_level_standard: analog::SignalLevelStandard,
            video_setup: analog::VideoSetup,
            sync_types: analog::SyncTypes,
            /// Whether the display supports supports serration on the vertical sync.
            serrations: bool,
        },

        Digital {
            color_bit_depth: digital::ColorBitDepth,
            standards: Option<digital::SupportedVideoInterfaces>,
        },
    }

    pub mod analog {
        /// "Signal, Level, Total" for analog displays.
        #[repr(C)]
        #[expect(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub enum SignalLevelStandard {
            /// 0.7 + 0.3 = 1.0 V
            _0700S_0300L_1000T,

            /// 0.714 + 0.286 = 1.0 V
            _0714S_0286L_1000T,

            /// 1.0 + 0.4 = 1.4 V
            _1000S_0400L_1400T,

            /// 0.7 + 0.0 = 0.7 V
            _0700S_0000L_0700T,
        }

        /// The type of video setup on an analog display.
        #[repr(C)]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub enum VideoSetup {
            BlackLevel,
            B2BOrPedestal,
        }

        /// Info about the supported synchronization types.
        ///
        /// `true` is supported, `false` is unsupported.
        #[repr(C)]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct SyncTypes {
            separate_sync_h_and_v: bool,
            composite_sync_horizontal: bool,
            composite_sync_green_video: bool,
        }
    }

    pub mod digital {
        /// The bit depth of a digital display.
        #[repr(C)]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub enum ColorBitDepth {
            /// The display didn't tell us!
            Undefined,
            D6Bits,
            D8Bits,
            D10Bits,
            D12Bits,
            D14Bits,
            D16Bits,
            /// unused as of v1.4
            Reserved,
        }

        /// The supported digital video interface standards for a digital display.
        #[repr(C)]
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct SupportedVideoInterfaces {
            dvi: bool,
            hdmi_a: bool,
            hdmi_b: bool,
            mddi: bool,
            displayport: bool,
        }
    }
}

/// The screen size or aspect ratio of a device, if applicable.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum SizeOrRatio {
    ScreenSize {
        vertical_cm: u8,
        horizontal_cm: u8,
    },

    AspectRatio {
        vertical: u8,
        horizontal: u8,
    },

    /// This device either does not report these values, or the display
    /// doesn't have a static value for them.
    Inapplicable,
}

pub mod feature_support {
    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct FeatureSupport {
        power_management: PowerManagement,
        //
        // IMPLEMENTATION NOTE: we get "color type" if VideoSignalInterface::Analog,
        // otherwise we check for encoding formats
        color_support: ColorSupport,

        /// Whether sRGB Standard is the default color space.
        srgb_std: bool,

        /// Whether the Preferred Timing Mode has info about the native
        /// pixel format and preferred refresh rate for the display.
        says_pixel_format_and_refresh: bool,

        /// Whether the display is continuous-frequency.
        is_continuous_freq: bool,
    }

    /// Supported power modes.
    ///
    /// Note that the modern standard, DPM, will just report Active Off as
    /// supported and the others as unsupported.
    ///
    /// `true` means supported; `false` indicates unsupported.
    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct PowerManagement {
        standby: bool,
        suspend: bool,
        active_off: bool,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum ColorSupport {
        Type(ColorType),
        EncodingFormats(ColorEncodingFormats),
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum ColorType {
        MonochromeOrGrayscale,
        RgbColor,
        NonRgbColor,
        Undefined,
    }

    #[repr(C)]
    #[expect(non_camel_case_types)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum ColorEncodingFormats {
        Rgb444,
        Rgb444_YCrCb444,
        Rgb444_YCrCb422,
        Rgb444_YCrCb444_YCrCb422,
    }
}
