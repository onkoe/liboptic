//! Note that this just parses the 13-byte ASCII strings from one of the
//! descriptors that uses it.

use arrayvec::ArrayString;

// TODO: ensure VESA LS-EXT (localization) compliance

/// Parses out a 13-byte-long string from the given display descriptor bytes.
#[tracing::instrument(skip_all)]
pub(crate) fn parse(input: &[u8; 18]) -> ArrayString<13> {
    // make an arraystring (string on the stack w/ static size)
    let mut string = ArrayString::new_const();

    // we don't use the first five bytes, and the other 13 are ascii chars.
    //
    // convert and push the ascii chars
    for c in &input[5..=17] {
        // we can early return if we hit the end of the string
        if *c == 0x0A {
            return string;
        }

        if let Some(valid_char) = char::from_u32(*c as u32) {
            tracing::trace!(
                "Converted ASCII char to UTF-8 successfully! (was `{c}`, now `{valid_char}`)"
            );
            string.push(valid_char);
        } else {
            tracing::warn!("Invalid ASCII -> UTF-8 character value (`{c}`). Skipping...");
        }
    }

    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::internal::*;

    #[test]
    fn std_sample_thisisatest_string() {
        logger();
        // note: first five in this array are for a data string descriptor
        let bytes = [
            0x00, 0x00, 0x00, 0xFE, 0x00, 0x54, 0x48, 0x49, 0x53, 0x49, 0x53, 0x41, 0x54, 0x45,
            0x53, 0x54, 0x0a, 0x20,
        ];
        let got = parse(&bytes);
        tracing::info!("got: {got}");

        let expected = ArrayString::from("THISISATEST").unwrap();
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
        let got = parse(&bytes);
        tracing::info!("got: {got}");

        let expected = ArrayString::from("A0123456789").unwrap();
        tracing::warn!("expected: {expected}");

        assert_eq!(got, expected);
    }
}
