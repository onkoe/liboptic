use bitvec::{
    field::BitField,
    order::{Lsb0, Msb0},
    view::BitView,
};

use crate::prelude::internal::*;

/// Parses out a Detailed Timing Definition from the given 18 bytes.
///
/// This must only ever be called when the first two bytes are [0x00, 0x01].
pub(super) fn parse(bytes: &[u8; 18]) -> DetailedTimingDefinition {
    // ensure the first two bytes are >= [0x00, 0x01]
    if [bytes[0], bytes[1]] == [0x00, 0x00] {
        unreachable!();
    }

    // grab various components of the def
    let pixel_clock = pixel_clock(&[bytes[0x00], bytes[0x01]]);
    let horizontal_addressable_video_px = horizontal_addr_video_px(bytes[0x04], bytes[0x02]);
    let horizontal_blanking_px = horizontal_blanking_px(bytes[0x04], bytes[0x03]);

    todo!()
}

/// Calcluates the pixel clock for the [0x00, 0x01] bytes
#[tracing::instrument]
fn pixel_clock(bytes: &[u8; 2]) -> u16 {
    bytes.view_bits::<Msb0>().load::<u16>()
}

/// Calculates the horizontal addressable video (in px).
#[tracing::instrument]
fn horizontal_addr_video_px(shared_byte: u8, byte: u8) -> u16 {
    // grab the upper four bits. that's the upper u4 part
    let upper = shared_byte.view_bits::<Lsb0>()[0x04..=0x07].load::<u8>();

    // combine it with the real byte
    let combined: u16 = bytemuck::must_cast([byte, upper]);
    combined
}

/// Calculates the horizontal addressable video (in px).
#[tracing::instrument]
fn horizontal_blanking_px(shared_byte: u8, byte: u8) -> u16 {
    // grab the upper four bits. that's the upper u4 part
    let upper = shared_byte.view_bits::<Lsb0>()[0x00..=0x03].load::<u8>();

    // combine it with the real byte
    let combined: u16 = bytemuck::must_cast([byte, upper]);
    combined
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_clock_ordering() {
        let one = [0x01, 0x00];
        assert_eq!(pixel_clock(&one), 1_u16);

        let max_minus_one = [0xFE, 0xFF];
        assert_eq!(pixel_clock(&max_minus_one), 0b1111_1111_1111_1110);
    }

    #[test]
    fn hoz_px_ordering() {
        logger();
        let shared = 0b1111_0000;
        let byte = 0b0000_0001;

        assert_eq!(
            horizontal_addr_video_px(shared, byte),
            0b0000_1111_0000_0001
        );
        assert_eq!(horizontal_blanking_px(shared, byte), 0b0000_0000_0000_0001);

        // the other's shouldn't matter. try again with another full shared
        let shared = 0b1111_1111;
        assert_eq!(
            horizontal_addr_video_px(shared, byte),
            0b0000_1111_0000_0001
        );
        assert_eq!(
            horizontal_addr_video_px(shared, byte),
            0b0000_1111_0000_0001
        );

        // we'll use another byte pattern here
        let shared = 0b1100_0001;
        let byte = 0b1111_0000;
        assert_eq!(
            horizontal_addr_video_px(shared, byte),
            0b0000_1100_1111_0000
        );
        assert_eq!(horizontal_blanking_px(shared, byte), 0b0000_0001_1111_0000);
    }
}
