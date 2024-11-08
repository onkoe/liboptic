use bitvec::{order::Lsb0, view::BitView};

use crate::prelude::internal::*;

/// Finds the supported "established timings" for this display.
///
/// Note that the timings are hardcoded, mostly to have a lowest common
/// denominator for hardware support.
#[tracing::instrument]
pub(crate) fn parse(input: &[u8]) -> EstablishedTimings {
    let i = est_i(input[0x23]);
    let ii = est_ii(input[0x24]);
    let m = manufacturer(input[0x25]);

    EstablishedTimings {
        i,
        ii,
        manufacturer_timings: m,
    }
}

/// Grabs established timings. Call with byte `0x23`.
#[tracing::instrument]
fn est_i(byte: u8) -> EstablishedTimingsI {
    let bits = byte.view_bits::<Lsb0>();
    EstablishedTimingsI {
        _720x400_70hz: bits[7],
        _720x400_88hz: bits[6],
        _640x480_60hz: bits[5],
        _640x480_67hz: bits[4],
        _640x480_72hz: bits[3],
        _640x480_75hz: bits[2],
        _800x600_56hz: bits[1],
        _800x600_60hz: bits[0],
    }
}

/// Grabs established timings. Call with byte `0x24`.
#[tracing::instrument]
fn est_ii(byte: u8) -> EstablishedTimingsII {
    let bits = byte.view_bits::<Lsb0>();
    EstablishedTimingsII {
        _800x600_72hz: bits[7],
        _800x600_75hz: bits[6],
        _832x624_75hz: bits[5],
        _1024x768_87hz_interlaced: bits[4],
        _1024x768_60hz: bits[3],
        _1024x768_70hz: bits[2],
        _1024x768_75hz: bits[1],
        _1280x1024_75hz: bits[0],
    }
}

/// Checks the manufacturer timing flags. This does not find the manufacturer's
/// timings themselves.
///
/// Call with byte `0x25`.
#[tracing::instrument]
fn manufacturer(byte: u8) -> ManufacturerTimings {
    let bits = byte.view_bits::<Lsb0>();
    ManufacturerTimings {
        _1152x870_75hz: bits[7],
        _6: bits[6],
        _5: bits[5],
        _4: bits[4],
        _3: bits[3],
        _2: bits[2],
        _1: bits[1],
        _0: bits[0],
    }
}
