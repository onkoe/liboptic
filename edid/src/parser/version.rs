use winnow::PResult;

use crate::structures::version::EdidVersion;

/// Parses out the EDID version info.
#[tracing::instrument]
pub(crate) fn parse(input: &[u8]) -> PResult<EdidVersion> {
    let version = input[0x12];
    let revision = input[0x13];

    // notify the user if the version/revisions aren't supported
    if version > crate::LATEST_SUPPORTED_VERSION || revision > crate::LATEST_SUPPORTED_VERSION {
        tracing::warn!(
            "This crate hasn't yet been tested above EDID v{}.{}.",
            crate::LATEST_SUPPORTED_VERSION,
            crate::LATEST_SUPPORTED_REVISION
        )
    }

    Ok(EdidVersion { version, revision })
}
