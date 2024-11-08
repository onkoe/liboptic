//! Established timings.

/// A collection of common timings for a device.
///
/// These are mostly legacy, maybe even obsolute, but still useful.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EstablishedTimings {
    pub i: EstablishedTimingsI,
    pub ii: EstablishedTimingsII,
    pub manufacturer_timings: ManufacturerTimings,
}

/* Established Timing I
7 720 x 400 @ 70Hz IBM, VGA
6 720 x 400 @ 88Hz IBM, XGA2
5 640 x 480 @ 60Hz IBM, VGA
4 640 x 480 @ 67Hz Apple, Mac II
3 640 x 480 @ 72Hz VESA
2 640 x 480 @ 75Hz VESA
1 800 x 600 @ 56Hz VESA
0 800 x 600 @ 60Hz VESA
*/
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EstablishedTimingsI {
    pub _720x400_70hz: bool,
    pub _720x400_88hz: bool,
    pub _640x480_60hz: bool,
    pub _640x480_67hz: bool,
    pub _640x480_72hz: bool,
    pub _640x480_75hz: bool,
    pub _800x600_56hz: bool,
    pub _800x600_60hz: bool,
}

/* Established Timing II
7 800 x 600 @ 72Hz VESA
6 800 x 600 @ 75Hz VESA
5 832 x 624 @ 75Hz Apple, Mac II
4 1024 x 768 @ 87Hz(I) IBM - Interlaced
3 1024 x 768 @ 60Hz VESA
2 1024 x 768 @ 70Hz VESA
1 1024 x 768 @ 75Hz VESA
0 1280 x 1024 @ 75Hz VESA
*/
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EstablishedTimingsII {
    pub _800x600_72hz: bool,
    pub _800x600_75hz: bool,
    pub _832x624_75hz: bool,
    pub _1024x768_87hz_interlaced: bool,
    pub _1024x768_60hz: bool,
    pub _1024x768_70hz: bool,
    pub _1024x768_75hz: bool,
    pub _1280x1024_75hz: bool,
}

/* Manufacturer's Timings
7 1152 x 870 @ 75Hz Apple, Mac II
6-0 Reserved for Manufacturer Specified Timings
*/

/// Timings specific to a manufacturer.
///
/// This isn't comprehensive - see standard timings and detailed timings
/// more info.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ManufacturerTimings {
    pub _1152x870_75hz: bool,
    pub _6: bool,
    pub _5: bool,
    pub _4: bool,
    pub _3: bool,
    pub _2: bool,
    pub _1: bool,
    pub _0: bool,
}
