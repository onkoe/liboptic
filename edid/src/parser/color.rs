use bitvec::prelude::*;
use fraction::Decimal;

use crate::color::{ColorCharacteristics, ColorCoordinate};

/// Finds the approximate color characteristics (coordinates) for this display.
#[tracing::instrument(skip_all)]
pub(super) fn parse(input: &[u8]) -> ColorCharacteristics {
    // yo head to page 28 (as of nov. 2024) to know what's going on
    let _0x19 = input[0x19].view_bits::<Lsb0>();
    let _0x1a = input[0x1A].view_bits::<Lsb0>();

    // 🔴️ red
    let rx = {
        let (rx1, rx0) = (_0x19[7], _0x19[6]);
        let rx_etc = input[0x1b];
        into_decimal(make_u10(rx0, rx1, rx_etc))
    };

    let ry = {
        let (ry1, ry0) = (_0x19[5], _0x19[4]);
        let ry_etc = input[0x1c];
        into_decimal(make_u10(ry0, ry1, ry_etc))
    };

    // 🟢️ green!
    let gx = {
        let (gx1, gx0) = (_0x19[3], _0x19[2]);
        let gx_etc = input[0x1D];
        into_decimal(make_u10(gx0, gx1, gx_etc))
    };

    let gy = {
        let (gy1, gy0) = (_0x19[1], _0x19[0]);
        let gy_etc = input[0x1E];
        into_decimal(make_u10(gy0, gy1, gy_etc))
    };

    // 🔵️ blue
    let bx = {
        let (bx1, bx0) = (_0x1a[7], _0x1a[6]);
        let bx_etc = input[0x1F];
        into_decimal(make_u10(bx0, bx1, bx_etc))
    };

    // blue y.
    let by = {
        let (by1, by0) = (_0x1a[5], _0x1a[4]);
        let by_etc = input[0x20];
        into_decimal(make_u10(by0, by1, by_etc))
    };

    // 🤍️ finally, we can do the white coords. <3 b/c the circle rendered weird
    let wx = {
        let (wx1, wx0) = (_0x1a[3], _0x1a[2]);
        let wx_etc = input[0x21];
        into_decimal(make_u10(wx0, wx1, wx_etc))
    };

    let wy = {
        let (wy1, wy0) = (_0x1a[1], _0x1a[0]);
        let wy_etc = input[0x22];
        into_decimal(make_u10(wy0, wy1, wy_etc))
    };

    // make the actual coords
    let red = ColorCoordinate::new(rx, ry);
    let green = ColorCoordinate::new(gx, gy);
    let blue = ColorCoordinate::new(bx, by);
    let white = ColorCoordinate::new(wx, wy);

    ColorCharacteristics {
        red,
        green,
        blue,
        white_point: white,
    }
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
    for (index, bit) in etc.view_bits::<Lsb0>().into_iter().enumerate() {
        bits.set(index + 2, *bit); // we need to start at bits[2]
    }
    tracing::trace!("bits now: {:?}", bits);

    // now we'll make it into a `u16`
    tracing::debug!("u10 created! it's: {}", bits.load_be::<u16>());
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

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use fraction::Decimal;

    use super::make_u10;
    use crate::{color::ColorCoordinate, logger, parser::color::into_decimal};

    /// it should be comprised only of my ones
    #[test]
    fn check_make_u10() {
        logger();
        let bit0 = true;
        let bit1 = true;
        let etc = u8::MAX;

        let result = make_u10(bit0, bit1, etc);
        assert_eq!(result, 0b11_1111_1111);
    }

    /// make sure the function is behaving according to spec
    #[test]
    fn into_decimal_endpts() {
        logger();
        let start = 0b00_0000_0000;
        let midpoint = 0b00_0001_1111;
        let end = 0b11_1111_1111;

        assert_eq!(into_decimal(start), Decimal::from(0));
        assert_eq!(
            into_decimal(midpoint),
            Decimal::from(31) / Decimal::from(1024)
        );
        assert_eq!(into_decimal(end), Decimal::from(1023) / Decimal::from(1024));
    }

    /// samples from the standard
    #[test]
    fn into_decimal_samples() {
        let a = 0b10_0111_0001;
        let b = 0b01_0011_1010;
        let c = 0b00_1001_1010;

        assert_eq!(into_decimal(a), Decimal::from_str("0.6103516").unwrap());
        assert_eq!(into_decimal(b), Decimal::from_str("0.3066406").unwrap());
        assert_eq!(into_decimal(c), Decimal::from_str("0.1503906").unwrap());
    }

    /// test against the (unfortunately rounded) edid-decode values
    #[test]
    fn dell_s2417dg_color() {
        logger();
        let input = crate::prelude::internal::raw_edid_by_filename("dell_s2417dg.raw.input");
        let colors = super::parse(&input);
        tracing::info!("colors: {:#?}", colors);

        assert!(eq(
            colors.red,
            ColorCoordinate::new(Decimal::from(0.6396), Decimal::from(0.3300))
        ));
        assert!(eq(
            colors.green,
            ColorCoordinate::new(Decimal::from(0.2998), Decimal::from(0.5996))
        ));
        assert!(eq(
            colors.blue,
            ColorCoordinate::new(Decimal::from(0.1503), Decimal::from(0.0595))
        ));
        assert!(eq(
            colors.white_point,
            ColorCoordinate::new(Decimal::from(0.3125), Decimal::from(0.3291))
        ));
    }

    /// helper func to compare the coords :)
    #[tracing::instrument]
    fn eq(c1: ColorCoordinate, c2: ColorCoordinate) -> bool {
        let f = |d1, d2| -> bool {
            let m = if d1 < d2 { d2 - d1 } else { d1 - d2 };
            m <= Decimal::from(0.0002)
        };

        f(c1.x, c2.x) && f(c1.y, c2.y)
    }
}
