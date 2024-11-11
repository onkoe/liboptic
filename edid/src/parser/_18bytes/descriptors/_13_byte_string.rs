//! Note that this just parses the 13-byte ASCII strings from one of the
//! descriptors that uses it.

use arrayvec::ArrayString;

use crate::prelude::internal::*;

// TODO: ensure VESA LS-EXT (localization) compliance

/// Parses out a 13-byte-long string from the given display descriptor bytes.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8; 18]) -> Result<ArrayString<13>, EdidError> {
    // make an arraystring (string on the stack w/ static size).
    //
    // we don't use the first five bytes, and the other 13 are ascii chars
    let bytes: &[u8; 13] = input[5..=17].try_into()?;

    let mut ascii_bytes: [u8; 13] = [0x20; 13];

    // fix up weird text
    for (idx, byte) in bytes.iter().enumerate() {
        // rust doesn't like it when you go over 128, as that leads to regional
        // adaptations of ascii. we'll skip any of those and warn the user.
        if *byte >= 128 {
            tracing::warn!(
                "Attempted to use regional character in ASCII string ({byte}). \
            This character will be replaced with `?`."
            );
            ascii_bytes[idx] = b'?';
        } else {
            ascii_bytes[idx] = *byte;
        }
    }

    ArrayString::from_byte_string(&ascii_bytes).map_err(|e| {
        tracing::error!(
            "Failed to make string from given 13-byte values: (err: {e}, values: {bytes:?})"
        );
        EdidError::ArrayStringError
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_sample_thisisatest_string() {
        logger();
        // note: first five in this array are for a data string descriptor
        let bytes = [
            0x00, 0x00, 0x00, 0xFE, 0x00, 0x54, 0x48, 0x49, 0x53, 0x49, 0x53, 0x41, 0x54, 0x45,
            0x53, 0x54, 0x0a, 0x20,
        ];
        let got = parse(&bytes).unwrap();
        tracing::info!("got: {got}");

        let expected = ArrayString::from_byte_string(b"THISISATEST\n ").unwrap();
        tracing::warn!("expected: {expected}");

        assert_eq!(got, expected);
    }

    #[test]
    fn std_sample_a0123456789_string() {
        logger();
        // note: from the serial number descriptor example
        let bytes = [
            0x00, 0x00, 0x00, 0xFF, 0x00, 0x41, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
            0x38, 0x39, 0x0a, 0x20,
        ];
        let got = parse(&bytes).unwrap();
        tracing::info!("got: {got}");

        let expected = ArrayString::from_byte_string(b"A0123456789\n ").unwrap();
        tracing::warn!("expected: {expected}");

        assert_eq!(got, expected);
    }
}
