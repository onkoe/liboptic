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
