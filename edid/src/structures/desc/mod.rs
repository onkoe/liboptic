use cvt_3_byte_timing::TimingCodeDesc;

use self::display_range_limits::DisplayRangeLimits;

use super::{color::ColorCoordinate, std_timings::STiming};

pub mod cvt_3_byte_timing;
pub mod display_range_limits;

/// A byte "string" comprised of alphanumerics.
pub type ByteStr13 = [u8; 13];

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum DisplayDescriptor {
    /// <= 13 alphanumeric characters of a serial number.
    ///
    /// TAG: 0xFF
    ProductSerial(ByteStr13),

    /// <= 13 alphanumeric characters of a data string.
    ///
    /// TAG: 0xFE
    DataString(ByteStr13),

    /// Info about the display's range limits.
    ///
    /// TAG: 0xFD
    DisplayRangeLimits(DisplayRangeLimits),

    /// The model name of the display product.
    ///
    /// TAG: 0xFC
    ProductName(ByteStr13),

    /// Two sets of additional white points in the color space.
    ///
    /// TAG: 0xFB
    ColorPointData {
        w1: ColorCoordinate,
        w2: ColorCoordinate,
    },

    /// Six additional Standard Timings. If the display supports more, they'll
    /// be found in an extension block.
    ///
    /// TAG: 0xFA
    StandardTimingIdentifications {
        _9: STiming,
        _10: STiming,
        _11: STiming,
        _12: STiming,
        _13: STiming,
        _14: STiming,
    },

    /// aka "Display Color Management".
    ///
    /// TAG: 0xF9
    DcmData {
        version_number: u8,

        red_a3: u16,
        red_a2: u16,

        green_a3: u16,
        green_a2: u16,

        blue_a3: u16,
        blue_a2: u16,
    },

    /// Coordinated Video Timings (CVT) used to define video timing modes that
    /// include horizontal and vertical pixel formats that are not defined in
    /// the VESA DMT >= v1.0r10.
    ///
    /// TAG: 0xF8
    Cvt3ByteTimingCodes {
        version_number: u8,

        /// The first byte code descriptor. It has the highest priority and
        /// will always be present.
        first: TimingCodeDesc,

        // lower prio; optional
        second: Option<TimingCodeDesc>,
        third: Option<TimingCodeDesc>,
        last: Option<TimingCodeDesc>,
    },

    /// Even more timings.
    ///
    /// TAG: 0xF7
    EstablishedTimingsIII {
        _640x350_85hz: bool,
        _640x400_85hz: bool,
        _720x400_85hz: bool,
        _640x480_85hz: bool,
        _848x480_60hz: bool,
        _800x600_85hz: bool,
        _1024x768_85hz: bool,
        _1152x864_75hz: bool,

        _1280x768_60hz_reduced: bool,
        _1280x768_60hz: bool,
        _1280x768_75hz: bool,
        _1280x768_85hz: bool,
        _1280x960_60hz: bool,
        _1280x960_85hz: bool,
        _1280x1024_60hz: bool,
        _1280x1024_85hz: bool,

        _1360x768_60hz: bool,
        _1440x900_60hz_reduced: bool,
        _1440x900_60hz: bool,
        _1440x900_75hz: bool,
        _1440x900_85hz: bool,
        _1400x1050_60hz_reduced: bool,
        _1400x1050_60hz: bool,
        _1400x1050_75hz: bool,

        _1400x1050_85hz: bool,
        _1680x1050_60hz_reduced: bool,
        _1680x1050_60hz: bool,
        _1680x1050_75hz: bool,
        _1680x1050_85hz: bool,
        _1600x1200_60hz: bool,
        _1600x1200_65hz: bool,
        _1600x1200_70hz: bool,

        _1600x1200_75hz: bool,
        _1600x1200_85hz: bool,
        _1792x1344_60hz: bool,
        _1792x1344_75hz: bool,
        _1856x1392_60hz: bool,
        _1856x1392_75hz: bool,
        _1920x1200_60hz_reduced: bool,
        _1920x1200_60hz: bool,

        _1920x1200_75hz: bool,
        _1920x1200_85hz: bool,
        _1920x1440_60hz: bool,
        _1920x1440_75hz: bool,
    },

    /// A descriptor indicating that the space wasn't used.
    ///
    /// TAG: 0x10
    DummyDescriptor,
}
