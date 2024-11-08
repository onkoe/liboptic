use crate::prelude::internal::*;

/// Finds the supported "established timings" for this display.
///
/// Note that the timings are hardcoded, mostly to have a lowest common
/// denominator for hardware support.
#[tracing::instrument]
pub(crate) fn parse(input: &[u8]) -> EstablishedTimings {
    todo!()
}

/// Grabs established timings. Call with byte `0x23`.
#[tracing::instrument]
fn est_i(byte: u8) -> EstablishedTimingsI {
    todo!()
}

/// Grabs established timings. Call with byte `0x24`.
#[tracing::instrument]
fn est_ii(byte: u8) -> EstablishedTimingsII {
    todo!()
}

/// Checks the manufacturer timing flags. This does not find the manufacturer's
/// timings themselves.
///
/// Call with byte `0x25`.
#[tracing::instrument]
fn manufacturer(byte: u8) -> ManufacturerTimings {
    todo!()
}
