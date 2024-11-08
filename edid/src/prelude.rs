pub(crate) mod internal {
    #[cfg(test)]
    pub(crate) use crate::parser::util::{edid_by_filename, logger, raw_edid_by_filename};

    // probably the most important part lol
    pub(crate) use crate::Edid;

    // structure modules
    pub(crate) use crate::structures::{
        _18bytes, basic_info, color, est_timings, extension, id, std_timings, version,
    };

    pub(crate) use crate::structures::basic_info::{
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

}
