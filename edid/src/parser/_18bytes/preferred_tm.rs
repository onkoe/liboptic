use bitvec::{
    array::BitArray,
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
        unreachable!("passed wrong 18 byte desc. please report this with logs.");
    }

    // grab various components of the def
    let pixel_clock_khz = pixel_clock(&[bytes[0], bytes[1]]);

    let horizontal_addressable_video_px = upper_nibble(bytes[4], bytes[2]);
    let horizontal_blanking_px = lower_nibble(bytes[4], bytes[3]);
    let vertical_addressable_video_lines = upper_nibble(bytes[7], bytes[5]);
    let vertical_blanking_lines = lower_nibble(bytes[7], bytes[6]);

    // the rest of these share two bits in [11], so i'll get that out now
    let shared_bits = bytes[11].view_bits::<Lsb0>();

    let horizontal_front_porch: u16 = {
        let upper = shared_bits[6..=7].load::<u8>();
        let lower = bytes[8];
        bytemuck::must_cast([lower, upper])
    }; // u12
    let horizontal_sync_pulse_width_px: u16 = {
        let upper = shared_bits[4..=5].load::<u8>();
        let lower = bytes[9];
        bytemuck::must_cast([lower, upper])
    }; // u12
    let vertical_front_porch_lines: u8 = {
        let upper_half = bytes[10].view_bits::<Lsb0>();
        let mut arr: BitArray<u8, Msb0> = BitArray::ZERO;

        // lower u4
        arr.set(0x05, upper_half[7]);
        arr.set(0x04, upper_half[6]);
        arr.set(0x03, upper_half[5]);
        arr.set(0x02, upper_half[4]);

        // upper u2
        arr.set(0x01, shared_bits[2]);
        arr.set(0x00, shared_bits[3]);

        arr.load::<u8>()
    }; // really a u6
    let vertical_sync_pulse_width_lines: u8 = {
        let lower_half = bytes[0x10].view_bits::<Lsb0>();
        let mut arr: BitArray<u8, Msb0> = BitArray::ZERO;

        // lower u4
        arr.set(5, lower_half[3]);
        arr.set(4, lower_half[2]);
        arr.set(3, lower_half[1]);
        arr.set(2, lower_half[0]);

        // upper u2
        arr.set(1, shared_bits[0]);
        arr.set(0, shared_bits[1]);

        arr.load::<u8>()
    };

    // video image size/border defs
    //
    let horizontal_addressable_video_size_mm = {
        let mm = upper_nibble(bytes[14], bytes[12]);
        if mm == 0 {
            None
        } else {
            Some(mm)
        }
    };
    let vertical_addressable_video_size_mm = {
        let mm = lower_nibble(bytes[14], bytes[13]);
        if mm == 0 {
            None
        } else {
            Some(mm)
        }
    };
    let horizontal_border_px = bytes[15];
    let vertical_border_lines = bytes[16];

    // signal defs
    let (signal_interface_type, stereo_support, sync_signal) = part_2(bytes[17]);

    DetailedTimingDefinition {
        pixel_clock_khz,
        horizontal_addressable_video_px,
        horizontal_blanking_px,
        vertical_addressable_video_lines,
        vertical_blanking_lines,
        horizontal_front_porch,
        horizontal_sync_pulse_width_px,
        vertical_front_porch_lines,
        vertical_sync_pulse_width_lines,
        horizontal_addressable_video_size_mm,
        vertical_addressable_video_size_mm,
        horizontal_border_px,
        vertical_border_lines,
        signal_interface_type,
        stereo_support,
        sync_signal,
    }
}

/// Calcluates the pixel clock for the [0x00, 0x01] bytes
#[tracing::instrument]
fn pixel_clock(bytes: &[u8; 2]) -> u16 {
    bytes.view_bits::<Msb0>().load::<u16>()
}

/// Calculates the combined `u12` value for a field that uses the upper nibble.
#[tracing::instrument]
fn upper_nibble(shared_byte: u8, byte: u8) -> u16 {
    // grab the upper four bits. that's the upper u4 part
    let upper = shared_byte.view_bits::<Lsb0>()[0x04..=0x07].load::<u8>();

    // combine it with the real byte
    let combined: u16 = bytemuck::must_cast([byte, upper]);
    combined
}

/// Calculates the combined `u12` value for a field that uses the lower nibble.
#[tracing::instrument]
fn lower_nibble(shared_byte: u8, byte: u8) -> u16 {
    // grab the upper four bits. that's the upper u4 part
    let upper = shared_byte.view_bits::<Lsb0>()[0x00..=0x03].load::<u8>();

    // combine it with the real byte
    let combined: u16 = bytemuck::must_cast([byte, upper]);
    combined
}

/// Finds the "part 2" section of the detailed timing definition.
fn part_2(byte: u8) -> (SignalInterfaceType, StereoViewingSupport, SyncSignal) {
    // grab it as bits
    let bits = byte.view_bits::<Lsb0>(); // lsb0 to make bits[0x07] = bit 7

    // construct the signal interface type
    let sit = if bits[7] {
        SignalInterfaceType::Interlaced
    } else {
        SignalInterfaceType::NonInterlaced
    };

    // check for stereo audio support
    let stereo_viewing = match (bits[6], bits[5], bits[0]) {
        (false, false, _) => StereoViewingSupport::NormalDisplay,
        (false, true, false) => StereoViewingSupport::FieldSequentialRight,
        (true, false, false) => StereoViewingSupport::FieldSequentialLeft,
        (false, true, true) => StereoViewingSupport::TwoWayInterleavedRight,
        (true, false, true) => StereoViewingSupport::TwoWayInterleavedLeft,
        (true, true, false) => StereoViewingSupport::FourWayInterleaved,
        (true, true, true) => StereoViewingSupport::SideBySide,
    };

    // grab the sync signal definitions.
    //
    // digital if bits[4], else analog
    let sync_signal = if bits[4] {
        SyncSignal::Digital(match (bits[3], bits[2], bits[1]) {
            (false, false, _) => DigitalSyncSignal::Composite,
            (false, true, _) => DigitalSyncSignal::CompositeSerrations,
            (true, false, false) => DigitalSyncSignal::SeparateNegVNegH,
            (true, false, true) => DigitalSyncSignal::SeparateNegVPosH,
            (true, true, false) => DigitalSyncSignal::SeparatePosVNegH,
            (true, true, true) => DigitalSyncSignal::SeparatePosVPosH,
        })
    } else {
        let bipolar = bits[3];
        let with_serrations = bits[2];
        let sync_mode = if bits[1] {
            AnalogSyncOn::Rgb
        } else {
            AnalogSyncOn::Green
        };

        SyncSignal::Analog {
            bipolar,
            with_serrations,
            sync_mode,
        }
    };

    (sit, stereo_viewing, sync_signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_pixel_clock_ordering() {
        let one = [0x01, 0x00];
        assert_eq!(pixel_clock(&one), 1_u16);

        let max_minus_one = [0xFE, 0xFF];
        assert_eq!(pixel_clock(&max_minus_one), 0b1111_1111_1111_1110);
    }

    #[test]
    fn check_nibble_ordering() {
        logger();
        let shared = 0b1111_0000;
        let byte = 0b0000_0001;

        assert_eq!(upper_nibble(shared, byte), 0b0000_1111_0000_0001);
        assert_eq!(lower_nibble(shared, byte), 0b0000_0000_0000_0001);

        // the other's shouldn't matter. try again with another full shared
        let shared = 0b1111_1111;
        assert_eq!(upper_nibble(shared, byte), 0b0000_1111_0000_0001);
        assert_eq!(lower_nibble(shared, byte), 0b0000_1111_0000_0001);

        // we'll use another byte pattern here
        let shared = 0b1100_0001;
        let byte = 0b1111_0000;
        assert_eq!(upper_nibble(shared, byte), 0b0000_1100_1111_0000);
        assert_eq!(lower_nibble(shared, byte), 0b0000_0001_1111_0000);
    }

    fn _sam02e3_2c47316eff13_preferred_tm() {
        logger();
        let path = "linuxhw_edid_EDID_Digital_Samsung_SAM02E3_2C47316EFF13.input";
        let input = edid_by_filename(path);
        let bytes: [u8; 18] = input[0x36..=0x47].try_into().unwrap();

        // parse the first descriptor (always a preferred timing mode!)
        let got = super::parse(&bytes);

        // prepare yourself
        let expected = DetailedTimingDefinition {
            pixel_clock_khz: 10650_u16,

            horizontal_addressable_video_px: 1440,
            horizontal_blanking_px: 80 + 152 + 232, // just sum the h values
            vertical_addressable_video_lines: 900_u16,
            vertical_blanking_lines: 3 + 6 + 25, // ...and the v values here

            horizontal_front_porch: 80,
            horizontal_sync_pulse_width_px: 152,
            vertical_front_porch_lines: 3,
            vertical_sync_pulse_width_lines: 6,

            horizontal_addressable_video_size_mm: Some(367),
            vertical_addressable_video_size_mm: Some(229),

            horizontal_border_px: 0,  // FIXME: not mentioned!
            vertical_border_lines: 0, // FIXME

            signal_interface_type: SignalInterfaceType::NonInterlaced,
            stereo_support: StereoViewingSupport::NormalDisplay,
            sync_signal: SyncSignal::Digital(DigitalSyncSignal::SeparatePosVNegH),
        };

        assert_eq!(got, expected);
    }
}
