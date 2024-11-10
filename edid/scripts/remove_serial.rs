//! reads a display's edid from disk and reads its serial code.
//!
//! if it's not zero, it replaces it with `[0x00, 0x01]` and saves the file.
//!
//! ```cargo
//! [dependencies]
//! tracing-subscriber = "0.3.18"
//! tracing = "0.1.40"
//!
//! ```

use std::{fs::File, io::Write as _};

fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // open up the file
    const FILENAME: &str = "some.raw.input";
    let path = path(FILENAME);
    let mut bytes = raw_edid_by_filename(&path);

    // ensure checksum calculation is correct by calculating the one that's
    // already there.
    //
    // if the old checksum isn't correct, we just warn the user and continue.
    if check_checksum(&bytes) {
        let old_checksum = make_checksum(&bytes[..0x7F]);
        assert_eq!(old_checksum, bytes[0x7F], "checksum calc is wrong!");
        tracing::debug!("Checksum calculation was correct. Continuing with changes...");
    } else {
        tracing::warn!(
            "The original EDID failed its checksum. \
        Continuing with changes anyways..."
        );
    }

    // mutate the bytes
    bytes[0x0C] = 0b0001;
    bytes[0x0D] = 0b0000;
    bytes[0x0E] = 0b0000;
    bytes[0x0F] = 0b0000;
    tracing::info!("Overwrote serial number bytes successfully.");

    // make a new checksum
    let checksum = make_checksum(&bytes);
    bytes[0x7F] = checksum;
    assert_eq!(
        add(&bytes[..=0x7F]) % 256,
        0x00,
        "adding all values should result in zero"
    );

    // save the file to disk
    let mut f = File::create(path).unwrap();
    f.write_all(&bytes).unwrap();
    tracing::info!("All done!");
}

/// Grabs a raw (not encoded) EDID from disk at `tests/assets/`
#[tracing::instrument]
pub(crate) fn raw_edid_by_filename(path: &std::path::PathBuf) -> Vec<u8> {
    std::fs::read(path).unwrap()
}

fn path(filename: &str) -> std::path::PathBuf {
    let cwd = std::env::current_dir().unwrap();
    if cwd.components().last().unwrap().as_os_str() == "scripts" {
        cwd.join("../tests/assets").join(filename)
    } else {
        cwd.join("tests/assets").join(filename)
    }
}

#[tracing::instrument(skip_all)]
fn make_checksum(input: &[u8]) -> u8 {
    let non_checksum_bytes = &input[..0x7F]; // excludes the last byte, which has the old checksum
    let total = add(non_checksum_bytes) % 256;
    (256 - total) as u8 // this MUST be <= 255, so always fits.
}

/// Adds all bytes in a slice.
fn add(bytes: &[u8]) -> u32 {
    bytes.iter().map(|b| *b as u32).sum::<u32>()
}

/// Checks that the given slice's first 128 bytes sum (with overflow) to zero.
#[tracing::instrument(skip_all)]
fn check_checksum(bytes: &[u8]) -> bool {
    let used = &bytes[..=0x7F];
    let sum = add(used);

    let res = sum % 256;
    if res != 0x00 {
        tracing::error!("The given byte slices failed its checksum.");
        false
    } else {
        true
    }
}
