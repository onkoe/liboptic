//! reads a display's edid from disk and reads its serial code.
//!
//! if it's not zero, it replaces it with `0x0001` and saves the file.

use std::{fs::File, io::Write as _};

// FIXME: currently doesn't account for checksum lol
fn main() {
    // open up the file
    const PATH: &str = "dell_s2417dg.raw.input";
    let mut info = raw_edid_by_filename(PATH);

    // mutate the bytes
    info[0x0C] = 0b0001;
    info[0x0D] = 0b0000;
    info[0x0E] = 0b0000;
    info[0x0F] = 0b0000;

    // save the file to disk
    let mut f = File::create(
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(PATH),
    )
    .unwrap();
    f.write_all(&info).unwrap();
}

/// Grabs a raw (not encoded) EDID from disk at `tests/assets/`
pub(crate) fn raw_edid_by_filename(name: &str) -> Vec<u8> {
    let path =
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(name);

    std::fs::read(path).unwrap()
}
