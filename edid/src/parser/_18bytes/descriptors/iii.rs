//! Established Timings III descriptor (tag: 0xf7)

use bitvec::{order::Lsb0, slice::BitSlice, view::BitView as _};

use crate::prelude::internal::*;

/// Parses the Established Timings II descriptor.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8; 18]) -> Result<DisplayDescriptor, EdidError> {
    // view bits of each byte
    let b6 = input[6].view_bits::<Lsb0>();
    let b7 = input[7].view_bits::<Lsb0>();
    let b8 = input[8].view_bits::<Lsb0>();
    let b9 = input[9].view_bits::<Lsb0>();
    let b10 = input[10].view_bits::<Lsb0>();
    let b11 = input[11].view_bits::<Lsb0>();

    warn(input, b11)?;

    Ok(DisplayDescriptor::EstablishedTimingsIII {
        // byte 6
        _640x350_85hz: b6[7],
        _640x400_85hz: b6[6],
        _720x400_85hz: b6[5],
        _640x480_85hz: b6[4],
        _848x480_60hz: b6[3],
        _800x600_85hz: b6[2],
        _1024x768_85hz: b6[1],
        _1152x864_75hz: b6[0],

        // byte 7
        _1280x768_60hz_reduced: b7[7],
        _1280x768_60hz: b7[6],
        _1280x768_75hz: b7[5],
        _1280x768_85hz: b7[4],
        _1280x960_60hz: b7[3],
        _1280x960_85hz: b7[2],
        _1280x1024_60hz: b7[1],
        _1280x1024_85hz: b7[0],

        // byte 8
        _1360x768_60hz: b8[7],
        _1440x900_60hz_reduced: b8[6],
        _1440x900_60hz: b8[5],
        _1440x900_75hz: b8[4],
        _1440x900_85hz: b8[3],
        _1400x1050_60hz_reduced: b8[2],
        _1400x1050_60hz: b8[1],
        _1400x1050_75hz: b8[0],

        // byte 9
        _1400x1050_85hz: b9[7],
        _1680x1050_60hz_reduced: b9[6],
        _1680x1050_60hz: b9[5],
        _1680x1050_75hz: b9[4],
        _1680x1050_85hz: b9[3],
        _1600x1200_60hz: b9[2],
        _1600x1200_65hz: b9[1],
        _1600x1200_70hz: b9[0],

        // byte 10
        _1600x1200_75hz: b10[7],
        _1600x1200_85hz: b10[6],
        _1792x1344_60hz: b10[5],
        _1792x1344_75hz: b10[4],
        _1856x1392_60hz: b10[3],
        _1856x1392_75hz: b10[2],
        _1920x1200_60hz_reduced: b10[1],
        _1920x1200_60hz: b10[0],

        // byte 11
        _1920x1200_75hz: b11[7],
        _1920x1200_85hz: b11[6],
        _1920x1440_60hz: b11[5],
        _1920x1440_75hz: b11[4],
    })
}

/// Checks for use of reserved fields.
#[tracing::instrument]
fn warn(input: &[u8; 18], b11: &BitSlice<u8>) -> Result<(), EdidError> {
    let b11_reserved = [b11[3], b11[2], b11[1], b11[0]];
    if b11_reserved != [false, false, false, false] {
        tracing::warn!("The 11th byte of the Est. Timings III desc block used reserved bits: `{b11_reserved:?}`");
    }

    let reserved: [u8; 6] = input[12..=17].try_into()?;

    if reserved != [0, 0, 0, 0, 0, 0] {
        tracing::warn!(
            "Some of the six reserved bytes of the Est. Timings III desc were used: `{reserved:?}`"
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// having these values is also pretty rare
    #[test]
    fn gsm7666_fe91a60d5b6e_est_timings_desc_iii() {
        logger();
        let name = "linuxhw_edid_Digital_Goldstar_GSM7666_FE91A60D5B6E.input";
        let input = edid_by_filename(name);

        let got = super::parse(&input[0x6c..0x7e].try_into().unwrap()).unwrap();
        tracing::info!("GOT {:#?}", got);

        // oh lord
        let expected = DisplayDescriptor::EstablishedTimingsIII {
            _640x350_85hz: false,
            _640x400_85hz: false,
            _720x400_85hz: false,
            _640x480_85hz: false,
            _848x480_60hz: false,
            _800x600_85hz: false,
            _1024x768_85hz: false,
            _1152x864_75hz: false,

            _1280x768_60hz_reduced: false,
            _1280x768_60hz: true,
            _1280x768_75hz: false,
            _1280x768_85hz: false,
            _1280x960_60hz: false,
            _1280x960_85hz: false,
            _1280x1024_60hz: true,
            _1280x1024_85hz: false,

            _1360x768_60hz: true,
            _1440x900_60hz_reduced: true,
            _1440x900_60hz: false,
            _1440x900_75hz: false,
            _1440x900_85hz: false,
            _1400x1050_60hz_reduced: true,
            _1400x1050_60hz: false,
            _1400x1050_75hz: false,

            _1400x1050_85hz: false,
            _1680x1050_60hz_reduced: true,
            _1680x1050_60hz: false,
            _1680x1050_75hz: false,
            _1680x1050_85hz: false,
            _1600x1200_60hz: true,
            _1600x1200_65hz: false,
            _1600x1200_70hz: false,

            _1600x1200_75hz: false,
            _1600x1200_85hz: false,
            _1792x1344_60hz: false,
            _1792x1344_75hz: false,
            _1856x1392_60hz: false,
            _1856x1392_75hz: false,
            _1920x1200_60hz_reduced: true,
            _1920x1200_60hz: false,

            _1920x1200_75hz: false,
            _1920x1200_85hz: false,
            _1920x1440_60hz: false,
            _1920x1440_75hz: false,
        };
        tracing::warn!("EXPECTED: {expected:#?}");

        assert_eq!(got, expected);
    }
}
