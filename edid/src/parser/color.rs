use bitvec::prelude::*;
use fraction::Decimal;

use crate::color::ColorCharacteristics;

#[tracing::instrument(skip_all)]
pub(super) fn parse(input: &[u8]) -> ColorCharacteristics {
    todo!()
}

/// Creates a "u10" (10 bit unsigned integer) in a `u16`.
#[tracing::instrument]
fn make_u10(bit0: bool, bit1: bool, etc: u8) -> u16 {
    // make a place to store them all
    let mut bits: BitArray<u16, Lsb0> = BitArray::ZERO;

    // ...store them all :)
    bits.set(0, bit0);
    bits.set(1, bit1);

    // we'll iterate over all the `etc` bits and store them in the other list.
    for (index, bit) in etc.view_bits::<Msb0>().into_iter().enumerate() {
        bits.set(index + 2, *bit); // we need to start at bits[2]
    }
    tracing::trace!("bits now: {:?}", bits);

    // now we'll make it into a `u16`
    bits.load_be::<u16>()
}

/// Properly converts the given "u10" value into a decimal, then divides it
/// by its length.
///
/// This makes a typical decimal. Do not call with greater than 1023 (u10's max).
#[tracing::instrument]
fn into_decimal(raw_value: u16) -> Decimal {
    debug_assert!(raw_value <= 0b11_1111_1111, "otherwise ur calling it wrong");
    let len = 2_u16.pow(10); // 10 binary digits
    Decimal::from(raw_value) / len
}
