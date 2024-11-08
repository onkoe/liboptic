use bitvec::{order::Lsb0, slice::BitSlice};
use fraction::{Decimal, GenericFraction};
use winnow::PResult;

use crate::structures::basic_info::{
    feature_support::{
        ColorEncodingFormats, ColorSupport, ColorType, FeatureSupport, PowerManagement,
    },
    vsi::{
        analog,
        digital::{ColorBitDepth, SupportedVideoInterface},
        VideoSignalInterface,
    },
    BasicDisplayInfo, SizeOrRatio,
};

#[tracing::instrument]
pub(super) fn parse(input: &[u8]) -> PResult<BasicDisplayInfo> {
    let _input_def = video_input_definition(input[0x14])?;
    todo!()
}

#[tracing::instrument]
fn video_input_definition(byte: u8) -> PResult<VideoSignalInterface> {
    // using lsb to keep the bit numbering consistent.
    let bits: &BitSlice<u8, Lsb0> = BitSlice::from_element(&byte);

    fn digital(bits: &BitSlice<u8, Lsb0>) -> VideoSignalInterface {
        let color_bit_depth = match (bits[6], bits[5], bits[4]) {
            (false, false, false) => ColorBitDepth::Undefined,
            (false, false, true) => ColorBitDepth::D6Bits,
            (false, true, false) => ColorBitDepth::D8Bits,
            (false, true, true) => ColorBitDepth::D10Bits,
            (true, false, false) => ColorBitDepth::D12Bits,
            (true, false, true) => ColorBitDepth::D14Bits,
            (true, true, false) => ColorBitDepth::D16Bits,
            (true, true, true) => ColorBitDepth::Reserved,
        };

        // digital interface
        //
        // check if it's even supported
        let digital_interface = if !bits[3] && !bits[2] && !bits[1] && !bits[0] {
            tracing::debug!("digitial interface is not reported.");
            None
        } else {
            Some(match (bits[3], bits[2], bits[1], bits[0]) {
                (false, false, false, true) => SupportedVideoInterface::Dvi,
                (false, false, true, false) => SupportedVideoInterface::HdmiA,
                (false, false, true, true) => SupportedVideoInterface::HdmiB,
                (false, true, false, false) => SupportedVideoInterface::Mddi,
                (false, true, false, true) => SupportedVideoInterface::DisplayPort,
                reserved => {
                    tracing::error!("Got an unexpected digital video interface standard bit layout: `{reserved:#?}`");
                    unreachable!();
                }
            })
        };

        VideoSignalInterface::Digital {
            color_bit_depth,
            supported_interface: digital_interface,
        }
    }

    fn analog(bits: &BitSlice<u8, Lsb0>) -> VideoSignalInterface {
        // check level standard
        let signal_level_standard = match (bits[6], bits[5]) {
            (false, false) => analog::SignalLevelStandard::_0700S_0300L_1000T,
            (false, true) => analog::SignalLevelStandard::_0714S_0286L_1000T,
            (true, false) => analog::SignalLevelStandard::_1000S_0400L_1400T,
            (true, true) => analog::SignalLevelStandard::_0700S_0000L_0700T,
        };

        // video setup
        let video_setup = if bits[4] {
            analog::VideoSetup::B2BOrPedestal
        } else {
            analog::VideoSetup::BlackLevel
        };

        // sync types
        let sync_types = analog::SyncTypes {
            separate_sync_h_and_v: bits[3],
            composite_sync_horizontal: bits[2],
            composite_sync_green_video: bits[1],
        };

        let serrations = bits[0];

        VideoSignalInterface::Analog {
            signal_level_standard,
            video_setup,
            sync_types,
            serrations,
        }
    }

    // 0 if analog, 1 if digital
    if bits[7] {
        Ok(digital(bits))
    } else {
        Ok(analog(bits))
    }
}

#[tracing::instrument(skip_all)]
fn size_or_ratio(input: &[u8]) -> Option<SizeOrRatio> {
    match (input[0x15], input[0x16]) {
        // when both are 0x00, the screen's size isn't given or may be dynamic
        (0x00, 0x00) => None,

        // if vertical is 0x00, then horizontal is the landscape aspect ratio
        (horizontal, 0x00) => {
            tracing::debug!("landscape aspect ratio, given: `0x{horizontal:x}` (`{horizontal}`)");
            let (hoz, vert) = make_ratio(horizontal)?;

            Some(SizeOrRatio::AspectRatio {
                horizontal: hoz,
                vertical: vert,
            })
        }

        // now if horizontal is 0x00, we know to expect portrait orientation
        (0x00, vertical) => {
            tracing::debug!("portrait aspect ratio, given: `0x{vertical:x}` (`{vertical}`)");
            let (vert, hoz) = make_ratio(vertical)?;

            Some(SizeOrRatio::AspectRatio {
                horizontal: hoz,
                vertical: vert,
            })
        }

        // both are greater than zero, so we've got two cm counts
        (horizontal_cm, vertical_cm) => Some(SizeOrRatio::ScreenSize {
            vertical_cm,
            horizontal_cm,
        }),
    }
}

/// Makes an aspect ratio with the rounded EDID val: `?.xyz` => xyz_u8 (`ar)`.
///
/// To get landscape, pattern match the return value as `(hoz, vert)`. For
/// portrait, it's `(vert, hoz)`.
#[tracing::instrument]
fn make_ratio(ar: u8) -> Option<(u16, u16)> {
    // note: these values are calculated by dividing one side by the other,
    // then rounding to three decimal places.
    //
    // that works because `ar` is just those remaining decimal digits.
    // however, that also limits the ratio from (1:1 to 3.55:1), which
    // doesn't account for loooong displays.
    //
    // in addition, 1:1 (square) displays are not representable.
    //
    // i believe this is a limitation of the standard. (hopefully fixed in
    // displayid!)
    Some(match ar {
        0x00 => unreachable!(),
        0x4F => (16, 9),
        0x3D => (16, 10),
        0x22 => (4, 3),
        0x1A => (5, 4),
        0x05 => (3, 2),
        134 => (21, 9),
        _ => {
            if ar == 255 {
                tracing::warn!(
                    "Attempted to find EDID aspect ratio for monitor with ratio at 3.55:1.\
                Note that this display may have a different aspect ratio."
                );
            }

            let horiz_ar = 100 + (ar as u16);
            let frac = GenericFraction::<u16>::new(horiz_ar, 100_u16);
            (*frac.numer()?, *frac.denom()?)
        }
    })
}

/// Gets the gamma value from the given input stream.
///
/// Note that if this is `None`, the display should provide an extension
/// containing the value.
#[tracing::instrument(skip_all)]
fn gamma(input: &[u8]) -> Option<Decimal> {
    let byte = input[0x17];
    tracing::debug!("Got byte: 0x{byte:x}");

    if byte == 0xFF {
        tracing::info!("Reported None. An extension with the gamma value should follow...");
        None
    } else {
        if byte == 0x00 {
            tracing::warn!(
                "EDID 1.4 does not provide a defintion for `gamma: 0x00`, \
                but this display is using that. This may result in an inaccurate \
                answer."
            );
        }

        // reverse from the standard: byte = (GAMMA x 100) – 100
        Some((Decimal::from(byte) + 100) / 100)
    }
}

#[tracing::instrument]
fn feature_support(input: &[u8]) -> FeatureSupport {
    // again, using `Lsb0` despite standard being Msb0.
    //
    // this lets me use their numbering
    let bits: &BitSlice<u8, Lsb0> = BitSlice::from_element(&input[0x18]);

    // build the power management (i.e. bools)
    let power_management = PowerManagement {
        standby: bits[7],
        suspend: bits[6],
        active_off: bits[5],
    };

    // get color based on if we're analog/digital...
    let color_support = if BitSlice::<u8, Lsb0>::from_element(&input[0x14])[7] {
        // digital gets a color encoding!
        let formats = match (bits[4], bits[3]) {
            (false, false) => ColorEncodingFormats::Rgb444,
            (false, true) => ColorEncodingFormats::Rgb444_YCrCb444,
            (true, false) => ColorEncodingFormats::Rgb444_YCrCb422,
            (true, true) => ColorEncodingFormats::Rgb444_YCrCb444_YCrCb422,
        };

        ColorSupport::EncodingFormats(formats)
    } else {
        // if we're analog, just check the color type.
        let ty = match (bits[4], bits[3]) {
            (false, false) => ColorType::MonochromeOrGrayscale,
            (false, true) => ColorType::RgbColor,
            (true, false) => ColorType::NonRgbColor,
            (true, true) => ColorType::Undefined,
        };

        ColorSupport::Type(ty)
    };

    // other feature support flags
    let srgb_std = bits[2];
    let says_pixel_format_and_refresh = bits[1];
    let is_continuous_freq = bits[0];

    FeatureSupport {
        power_management,
        color_support,
        srgb_std,
        says_pixel_format_and_refresh,
        is_continuous_freq,
    }
}

#[cfg(test)]
mod tests {
    use fraction::Decimal;

    use crate::{
        parser::{
            basic_info::make_ratio,
            util::{edid_by_filename, logger},
        },
        structures::basic_info::{
            vsi::{
                digital::{ColorBitDepth, SupportedVideoInterface},
                VideoSignalInterface,
            },
            SizeOrRatio,
        },
    };

    #[test]
    fn dell_s2417dg_vsi() {
        logger();
        let input = crate::prelude::internal::raw_edid_by_filename("dell_s2417dg.raw.input");
        let got = super::video_input_definition(input[0x14]).unwrap();

        let expected = VideoSignalInterface::Digital {
            color_bit_depth: ColorBitDepth::D8Bits,
            supported_interface: Some(SupportedVideoInterface::DisplayPort),
        };

        assert_eq!(got, expected);
    }

    #[test]
    fn that_guys_laptop_vsi() {
        logger();
        let input = crate::prelude::internal::edid_by_filename("1.input");
        let got = super::video_input_definition(input[0x14]).unwrap();

        let expected = VideoSignalInterface::Digital {
            color_bit_depth: ColorBitDepth::D6Bits,
            supported_interface: None,
        };

        assert_eq!(got, expected);
    }

    #[test]
    fn dell_s2417dg_sizeratio() {
        logger();
        let input = crate::prelude::internal::raw_edid_by_filename("dell_s2417dg.raw.input");
        let got = super::size_or_ratio(&input).unwrap();

        let expected = SizeOrRatio::ScreenSize {
            vertical_cm: 30,
            horizontal_cm: 53,
        };

        assert_eq!(got, expected);
    }

    #[test]
    fn that_guys_laptop_sizeratio() {
        logger();
        let input = crate::prelude::internal::edid_by_filename("1.input");
        let got = super::size_or_ratio(&input).unwrap();

        let expected = SizeOrRatio::ScreenSize {
            vertical_cm: 17,
            horizontal_cm: 29,
        };

        assert_eq!(got, expected);
    }

    #[test]
    fn display_w_aspect_ratio() {
        logger();
        let input = edid_by_filename("linuxhw_edid_Digital_BOE_BOE07AF_BD22D8FDF96B.input");
        let got = super::size_or_ratio(&input).unwrap();

        let expected = SizeOrRatio::AspectRatio {
            horizontal: 16,
            vertical: 9,
        };

        assert_eq!(got, expected);
    }

    #[test]
    fn lotta_aspect_ratios() {
        logger();
        let _get_ar_val = |x: u8, y: u8| ((x as f32 / y as f32) * 100.0) - 99.0;
        // panic!("{}", _get_ar_val(33, 23));

        assert_eq!(make_ratio(79_u8), Some((16, 9)));
        assert_eq!(make_ratio(61_u8), Some((16, 10)));
        assert_eq!(make_ratio(34_u8), Some((4, 3)));
        assert_eq!(make_ratio(26_u8), Some((5, 4)));
        assert_eq!(make_ratio(5_u8), Some((3, 2)));
        assert_eq!(make_ratio(134_u8), Some((21, 9)));

        // some weird ones i pulled outta my ass
        assert_eq!(make_ratio(16_u8), Some((29, 25)));
        assert_eq!(make_ratio(45_u8), Some((29, 20)));
        assert_eq!(make_ratio(255_u8), Some((71, 20))); // i would so buy this
    }

    #[test]
    fn dell_s2417dg_gamma() {
        logger();
        let input = crate::prelude::internal::raw_edid_by_filename("dell_s2417dg.raw.input");
        let got = super::gamma(&input).unwrap();
        let expected = Decimal::from(2.20);

        assert_eq!(got, expected);
    }

    #[test]
    fn that_guys_laptop_gamma() {
        logger();
        let input = crate::prelude::internal::edid_by_filename("1.input");
        let got = super::gamma(&input).unwrap();
        let expected = Decimal::from(2.20);

        assert_eq!(got, expected);
    }

    #[test]
    fn _93d328459ff6_gamma() {
        logger();
        let input = crate::prelude::internal::edid_by_filename(
            "linuxhw_edid_EDID_Digital_Sony_SNY05FA_93D328459FF6.input",
        );
        let got = super::gamma(&input).unwrap();
        let expected = Decimal::from(1.0);

        assert_eq!(got, expected);
    }
}
