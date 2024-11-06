use bitvec::{order::Lsb0, slice::BitSlice};
use winnow::PResult;

use crate::structures::basic_info::{
    feature_support::FeatureSupport,
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
                    unreachable!()
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
        Ok(digital(&bits))
    } else {
        Ok(analog(&bits))
    }
}

#[tracing::instrument]
fn size_or_ratio(input: &[u8]) -> PResult<SizeOrRatio> {
    todo!()
}

#[tracing::instrument]
fn reports_gamma(input: &[u8]) -> bool {
    todo!()
}

#[tracing::instrument]
fn feature_support(input: &[u8]) -> PResult<FeatureSupport> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::structures::basic_info::vsi::{
        digital::{ColorBitDepth, SupportedVideoInterface},
        VideoSignalInterface,
    };

    #[test]
    fn dell_s2417dg_vsi() {
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
        let input = crate::prelude::internal::edid_by_filename("1.input");
        let got = super::video_input_definition(input[0x14]).unwrap();

        let expected = VideoSignalInterface::Digital {
            color_bit_depth: ColorBitDepth::D6Bits,
            supported_interface: None,
        };

        assert_eq!(got, expected);
    }
}
