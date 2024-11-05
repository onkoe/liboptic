//! Eighteen-byte descriptors.

use crate::structures::desc::DisplayDescriptor;

/// A collection of "18-byte descriptors".
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EighteenByteDescriptors {
    /// The video timing mode that produces the "best quality image"
    /// according to the display's manufacturer.
    preferred_timing_mode: timing::DetailedTimingDefinition,

    /// Three additional blocks with information about the display.
    blocks: [EighteenByteBlock; 3],
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
        pixel_clock_khz: u16,

        horizontal_addressable_video_px: u16,
        horizontal_blanking_px: u16,
        vertical_addressable_video_lines: u16,
        vertical_blanking_lines: u16,

        horizontal_front_porch: u16,
        horizontal_sync_pulse_width_px: u16,
        vertical_front_porch_lines: u16,
        vertical_sync_pulse_width_lines: u16,
        //
        //
        //
        // video image size/border defs!
        //
        horizontal_addressable_video_size_mm: u16,
        vertical_addressable_video_size_mm: u16,
        horizontal_border_px: u8,
        vertical_border_lines: u8,
        //
        //
        //
        // signal (?) defs!
        //
        signal_interface_type: SignalInterfaceType,
        stereo_support: StereoViewingSupport,
        sync_signal: SyncSignal,
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
        FourWayInterlaced,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum SyncSignal {
        Analog(AnalogSyncSignal),
        Digital(DigitalSyncSignal),
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum AnalogSyncSignal {
        Composite,
        Bipolar,
        BipolarSerrationsSyncOnGreen,
        BipolarSerrationsSyncRgb,
    }

    #[repr(C)]
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub enum DigitalSyncSignal {
        Composite,
        CompositeSerrations,
        SeparateNegV,
        SeparatePosVNegH,
        SeparatePosVPosH,
    }
}
