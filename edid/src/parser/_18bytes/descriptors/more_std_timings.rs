use crate::prelude::internal::*;

use crate::parser::std_timings::one;

/// Parses... six more standard timings. In case your display is using all
/// those.
#[tracing::instrument(skip(input))]
pub(crate) fn parse(input: &[u8; 18]) -> Result<DisplayDescriptor, EdidError> {
    // error if weird header value
    let header = &input[0..5];
    if header != [0x00, 0x00, 0x00, 0xFA, 0x00] {
        tracing::error!("Given descriptor data had a malformed header: {header:x?}");
        return Err(EdidError::DescriptorUnexpectedHeader(header.try_into()?));
    }

    // check if the last value matches the expected one. i'll let this one fly
    if let Some(last) = input.last() {
        if *last != 0x0A {
            tracing::warn!(
                "Standard timings display descriptor used reserved byte 17. (value: {last}"
            );
        }
    }

    Ok(DisplayDescriptor::StandardTimingIdentifications {
        _9: one(&input[5..7]),
        _10: one(&input[7..9]),
        _11: one(&input[9..11]),
        _12: one(&input[11..13]),
        _13: one(&input[13..15]),
        _14: one(&input[15..17]),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// for whatever reason, msi decided to add this fully-empty descriptor
    /// to this file.
    ///
    /// might be useful for regression testing or etc.
    #[test]
    fn _msiaf82_4b2991d4299a_more_std_timings() {
        logger();
        let path = "linuxhw_edid_Digital_MSI_MSIAF82_4B2991D4299A.input";
        let input = edid_by_filename(path);

        let got = parse(&input[0x48..0x5a].try_into().unwrap()).unwrap();
        tracing::info!("GOT: {got:#?}");

        let expected = DisplayDescriptor::StandardTimingIdentifications {
            _9: None,
            _10: None,
            _11: None,
            _12: None,
            _13: None,
            _14: None,
        };
        tracing::warn!("EXPECTED: {expected:#?}");

        assert_eq!(got, expected);
    }

    #[test]
    fn len0017_3af8b597ecb9_more_std_timings() {
        logger();
        let path = "linuxhw_edid_Digital_Lenovo_LEN0017_3AF8B597ECB9.input";
        let input = edid_by_filename(path);

        let got = parse(&input[0x48..0x5a].try_into().unwrap()).unwrap();
        tracing::info!("GOT: {got:#?}");

        let expected = DisplayDescriptor::StandardTimingIdentifications {
            _9: Some(STiming {
                horizontal_addr_pixel_ct: 296,
                aspect_ratio: StandardAspectRatio::_16_10,
                field_refresh_rate: 76,
            }),
            _10: Some(STiming {
                horizontal_addr_pixel_ct: 632,
                aspect_ratio: StandardAspectRatio::_16_10,
                field_refresh_rate: 77,
            }),
            _11: None,
            _12: None,
            _13: None,
            _14: None,
        };
        tracing::warn!("EXPECTED: {expected:#?}");

        assert_eq!(got, expected);
    }

    /// this one just has a single listing
    #[test]
    fn aoc0320_455954e7ca14_more_std_timings() {
        logger();
        let path = "linuxhw_edid_Analog_AOC_AOC0320_455954E7CA14.input";
        let input = edid_by_filename(path);

        let got = parse(&input[0x6c..0x7e].try_into().unwrap()).unwrap();
        tracing::info!("GOT: {got:#?}");

        let expected = DisplayDescriptor::StandardTimingIdentifications {
            _9: Some(STiming {
                horizontal_addr_pixel_ct: 1600,
                aspect_ratio: StandardAspectRatio::_16_9,
                field_refresh_rate: 60,
            }),
            _10: None,
            _11: None,
            _12: None,
            _13: None,
            _14: None,
        };
        tracing::warn!("EXPECTED: {expected:#?}");

        assert_eq!(got, expected);
    }

    /// almost compliant!
    #[test]
    fn hjw0000_f67302f2ed4c_more_std_timings() {
        logger();
        let path = "linuxhw_edid_Digital_Others_HJW0000_F67302F2ED4C.input";
        let input = edid_by_filename(path);

        let got = parse(&input[0x6c..0x7e].try_into().unwrap()).unwrap();
        tracing::info!("GOT: {got:#?}");

        let expected = DisplayDescriptor::StandardTimingIdentifications {
            _9: Some(STiming {
                horizontal_addr_pixel_ct: 1152,
                aspect_ratio: StandardAspectRatio::_4_3,
                field_refresh_rate: 60,
            }),
            _10: Some(STiming {
                horizontal_addr_pixel_ct: 1280,
                aspect_ratio: StandardAspectRatio::_16_10,
                field_refresh_rate: 75,
            }),
            _11: Some(STiming {
                horizontal_addr_pixel_ct: 1360,
                aspect_ratio: StandardAspectRatio::_16_9,
                field_refresh_rate: 60,
            }),
            _12: Some(STiming {
                horizontal_addr_pixel_ct: 1360,
                aspect_ratio: StandardAspectRatio::_16_9,
                field_refresh_rate: 60,
            }),
            _13: Some(STiming {
                horizontal_addr_pixel_ct: 1400,
                aspect_ratio: StandardAspectRatio::_4_3,
                field_refresh_rate: 60,
            }),
            _14: Some(STiming {
                horizontal_addr_pixel_ct: 1600,
                aspect_ratio: StandardAspectRatio::_16_9,
                field_refresh_rate: 60,
            }),
        };
        tracing::warn!("EXPECTED: {expected:#?}");

        assert_eq!(got, expected);
    }
}
