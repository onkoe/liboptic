//! Eighteen-byte descriptors.

use crate::structures::desc::DisplayDescriptor;

/// A collection of "18-byte descriptors".
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EighteenByteDescriptors {
    /// The video timing mode that produces the "best quality image"
    /// according to the display's manufacturer.
    pub preferred_timing_mode: timing::DetailedTimingDefinition,

    /// Three additional blocks with information about the display.
    pub blocks: [EighteenByteBlock; 3],
}

/// An eighteen-byte block containing either a detailed timing or display
/// descriptor.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum EighteenByteBlock {
    Timing(timing::DetailedTimingDefinition),
    Display(DisplayDescriptor),
}

pub mod timing {

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct DetailedTimingDefinition {
        // timing defs!
        //
        /// The pixel clock of the device in kHz.
        ///
        /// Range is [10, 65535] KHz, in steps of 10 kHz.
        pub pixel_clock_khz: u16,

        pub horizontal_addressable_video_px: u16,
        pub horizontal_blanking_px: u16,
        pub vertical_addressable_video_lines: u16,
        pub vertical_blanking_lines: u16,

        /// From blanking start to start of sync. Range is [0, 1023] px.
        ///
        /// Sometimes known as the "horizontal sync offset".
        pub horizontal_front_porch: u16,
        /// From the end of the front porch to the start of the back porch.
        /// Range is [0, 1023] px.
        pub horizontal_sync_pulse_width_px: u16,
        /// From blanking start to start of sync. Range is [0, 63] lines.
        ///
        /// Sometimes known as the "vertical sync offset".
        pub vertical_front_porch_lines: u8,
        /// From the end of the front porch to the start of the back porch.
        /// Range is [0, 63] lines.
        pub vertical_sync_pulse_width_lines: u8,

        // video image size/border defs!
        //
        //
        /// The horizontal screen size. May not be present for some kinds of
        /// devices. Range is [0, 4095] mm.
        ///
        /// Note that this defines the video size of **the displayed image**,
        /// not where light can be controlled.
        pub horizontal_addressable_video_size_mm: Option<u16>,
        /// The vertical screen size. May not be present for some kinds of
        /// devices. Range is [0, 4095] mm.
        ///
        /// Note that this defines the video size of **the displayed image**,
        /// not where light can be controlled.
        pub vertical_addressable_video_size_mm: Option<u16>,
        pub horizontal_border_px: u8,
        pub vertical_border_lines: u8,
        //
        //
        //
        // signal (?) defs!
        //
        pub signal_interface_type: SignalInterfaceType,
        pub stereo_support: StereoViewingSupport,
        pub sync_signal: SyncSignal,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum SignalInterfaceType {
        /// 1 frame = 1 field
        NonInterlaced,
        /// 1 frame = 2 fields
        Interlaced,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum StereoViewingSupport {
        NormalDisplay,
        FieldSequentialRight,
        FieldSequentialLeft,
        TwoWayInterleavedRight,
        TwoWayInterleavedLeft,
        FourWayInterleaved,
        SideBySide,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum SyncSignal {
        Analog {
            bipolar: bool,
            with_serrations: bool,
            sync_mode: AnalogSyncOn,
        },

        Digital(DigitalSyncSignal),
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum AnalogSyncOn {
        Green,
        Rgb,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum DigitalSyncSignal {
        Composite,
        CompositeSerrations,
        SeparateNegVNegH,
        SeparateNegVPosH,
        SeparatePosVNegH,
        SeparatePosVPosH,
    }
}
