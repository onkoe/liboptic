use crate::prelude::internal::*;

const EDID_HEADER: [u8; 8] = [0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00];

#[tracing::instrument(skip_all)]
pub(super) fn parse(input: &[u8]) -> Result<(), EdidError> {
    // header is exactly 8 bytes long
    if input.len() < EDID_HEADER.len() {
        tracing::error!("the input is too small, so can't contain a header.");
        return Err(EdidError::HeaderTooShort {
            real_len: input.len() as u8,
            expected_len: EDID_HEADER.len() as u8,
        });
    }
    // we want to ensure the edid has the required (static) header
    let real_header = &input[0..=7];
    if real_header != EDID_HEADER {
        tracing::error!(
            "header does not match the expected. (real: {:#?})",
            &real_header
        );

        return Err(EdidError::NoHeader);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// make sure we don't panic when the header's empty. it should just be an
    /// `Err`
    #[test]
    fn empty() {
        logger();
        let empty_input = [];
        let result = parse(empty_input.as_slice());

        assert!(result.is_err());
    }

    /// an edid with a valid header
    #[test]
    fn good() {
        logger();
        let input = edid_by_filename("1.input");
        parse(input.as_slice()).unwrap();
    }

    /// an edid with a wrong header
    #[test]
    fn bad() {
        logger();
        let input = edid_by_filename("bad/bad.1.input");
        _ = parse(input.as_slice()).unwrap_err();
    }
}
