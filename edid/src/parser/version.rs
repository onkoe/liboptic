use crate::structures::version::EdidVersion;

use crate::prelude::internal::*;

/// Parses out the EDID version info.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8]) -> Result<EdidVersion, EdidError> {
    let version = input[0x12];
    let revision = input[0x13];

    // notify the user if the version/revisions aren't supported
    if version > crate::LATEST_SUPPORTED_VERSION || revision > crate::LATEST_SUPPORTED_VERSION {
        tracing::warn!(
            "This crate hasn't yet been tested above EDID v{}.{}.",
            crate::LATEST_SUPPORTED_VERSION,
            crate::LATEST_SUPPORTED_REVISION
        );
    }

    Ok(EdidVersion { version, revision })
}

#[cfg(test)]
mod tests {
    #[test]
    fn vnr_dell_s2417dg() {
        let input = crate::prelude::internal::raw_edid_by_filename("dell_s2417dg.raw.input");
        let vnr = super::parse(&input).unwrap();

        assert_eq!(vnr.version, 0x1);
        assert_eq!(vnr.revision, 0x4);
    }
}
