use bitvec::{order::Lsb0, view::BitView};
use color::ColorCoordinate;
use fraction::Decimal;

use crate::{
    parser::color::{into_decimal, make_u10},
    prelude::internal::*,
    structures::desc::color_point::WhitePoint,
};

/// Parses out chromaticity coordinates for the given color point descriptor.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8; 18]) -> DisplayDescriptor {
    let w1 = make_white_point(1, input);
    let w2 = make_white_point(2, input);

    DisplayDescriptor::ColorPointData { w1, w2 }
}

/// Creates a gamma decimal from the given raw value.
#[tracing::instrument]
fn gamma(raw: u8) -> Decimal {
    (Decimal::from(raw) + Decimal::from(100)) / Decimal::from(100)
}

/// Makes a white point given its `wi`, which is either 1 or 2.
#[tracing::instrument(skip(input))]
fn make_white_point(wi: u8, input: &[u8; 18]) -> WhitePoint {
    // convert index to usize.
    //
    // this also determines if we'll "shift" the parsing for the second value.
    // in other words, we avoid repeating all those indices twice
    let shift = match wi {
        1 => 0_usize,
        2 => 5_usize,
        weird => unreachable!("got a weird `wi` value: `{}`", weird),
    };

    // the index location is 5 on w1 and 10 on w2.
    let index = input[5 + shift];
    if index == 0x00 {
        tracing::warn!("First white point value is using a reserved ID! `{index}`");
    }

    // shared coord bits
    let shared_bits = input[6 + shift].view_bits::<Lsb0>();

    let wx = {
        let lower = &[shared_bits[3], shared_bits[2]];
        let upper = input[7 + shift];

        let coord_u10 = make_u10(lower[1], lower[0], upper);
        // let coord_u10 = make_u10(lower[0], lower[1], upper);
        into_decimal(coord_u10)
    };

    let wy = {
        let lower = &[shared_bits[1], shared_bits[0]];
        let upper = input[8 + shift];

        let coord_u10 = make_u10(lower[0], lower[1], upper);
        // let coord_u10 = make_u10(lower[0], lower[1], upper);
        into_decimal(coord_u10)
    };

    let coord = ColorCoordinate { x: wx, y: wy };

    let gamma = {
        let raw = input[9 + shift];
        if raw == 0xFF {
            None
        } else {
            Some(gamma(raw))
        }
    };

    WhitePoint {
        index_number: index,
        coord,
        gamma,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// note: out of ~100k reports, there are ZERO displays that used this
    /// block.
    ///
    /// as such, i'll just make up my own test and make sure it looks ok...
    #[test]
    fn _fictional_color_point() {
        logger();

        // again, this isn't real data
        let data = [
            // indicate that this is a valid color point descriptor
            0x00,
            0x00,
            0x00,
            0xFB,
            0x00,
            ///////////////
            //
            // start with w1
            1,           // white point index: 1
            0b0000_1100, // w1 shared (x: b11, y: b00)
            0b0000_1111, // w1x has its lower bits set only. that's 0b1111_11
            0b1000_0000, // w1y just has its highest bit set. => 0b10_0000_0000
            254,         // gamma is the highest allowed, 3.54
            ///////////////
            //
            // ok now here's w2
            2,
            0b0000_0001, // shared (x: b00, y: b01),
            0b0100_0000, // x: only has its second-highest bit
            0b1111_0001, // y
            23,          // gamma of 1.23
            ///////////////
            //
            // unused/reserved bytes
            0xff,
            0x0a,
            0x0a,
        ];
        let got = parse(&data);
        tracing::info!("GOT: \n{got:#?}");

        let expected = DisplayDescriptor::ColorPointData {
            w1: WhitePoint {
                index_number: 1,
                coord: ColorCoordinate::new(Decimal::from(63) / 1024, Decimal::from(1) / 2),
                gamma: Some(Decimal::from(3.54)),
            },
            w2: WhitePoint {
                index_number: 2,

                /* y is weird. so i'll visualize the bits

                <SMALLEST>
                    1
                    0

                    1
                    0
                    0
                    0

                    1
                    1
                    1
                    1
                <BIGGEST>

                */
                coord: ColorCoordinate::new(Decimal::from(1) / 4, Decimal::from(965) / 1024),
                gamma: Some(Decimal::from(1.23)),
            },
        };
        tracing::warn!("EXPECTED: \n{expected:#?}");

        assert_eq!(got, expected);
    }
}
