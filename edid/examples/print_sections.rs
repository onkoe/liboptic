//! reads a display's edid from disk and prints its sections.

use colored::Colorize;

fn main() {
    // open up the file
    const PATH: &str = "linuxhw_edid_EDID_Digital_Samsung_SAM02E3_2C47316EFF13.input";
    let info = edid_by_filename(PATH);

    use std::format as f;

    println!("for file: `{PATH}`...\n");

    println!("{}", f!("Header: {:x?}", &info[0x00..0x08]).blue());
    println!(
        "{}",
        f!("Vendor & Product Identification: {:x?}", &info[0x08..0x12]).green()
    );
    println!(
        "{}",
        f!(
            "EDID Structure Version & Revision: {:x?}",
            &info[0x12..0x14]
        )
        .magenta()
    );
    println!(
        "{}",
        f!(
            "Basic Display Parameters & Features: {:x?}",
            &info[0x14..0x19]
        )
        .red()
    );
    println!(
        "{}",
        f!("Color Characteristics: {:x?}", &info[0x19..0x23])
            .bright_cyan()
            .on_black()
    );
    println!(
        "{}",
        f!("Established Timings: {:x?}", &info[0x23..0x26])
            .white()
            .on_black()
    );
    println!(
        "{}",
        f!(
            "Standard Timings: Identification 1 → 8: {:x?}",
            &info[0x26..0x36]
        )
        .bright_red()
    );
    println!(
        "{}\n{}\n{}\n{}",
        f!(
            "18 Byte Data Block 1 ({:x}): {:x?}",
            info[0x39],
            &info[0x36..0x48]
        )
        .green()
        .on_black(),
        f!(
            "18 Byte Data Block 2 ({:x}): {:x?}",
            info[0x4b],
            &info[0x48..0x5a]
        )
        .green()
        .on_bright_black(),
        f!(
            "18 Byte Data Block 3 ({:x}): {:x?}",
            info[0x5d],
            &info[0x5a..0x6c]
        )
        .green()
        .on_black(),
        f!(
            "18 Byte Data Block 4 ({:x}): {:x?}",
            info[0x6f],
            &info[0x6c..0x7e]
        )
        .green()
        .on_bright_black()
    );
    println!(
        "{}",
        f!("Extension Block Count N: {:x?}", &info[0x7e]).bright_blue()
    );
    println!(
        "{}",
        f!("Checksum C: {:x?}", &info[0x7f]).yellow().on_black()
    );
}

/// Grabs a raw (not encoded) EDID from disk at `tests/assets/`
#[tracing::instrument]
pub(crate) fn raw_edid_by_filename(name: &str) -> Vec<u8> {
    let path =
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(name);

    std::fs::read("/sys/class/drm/card1-DP-3/edid").unwrap()
}

/// Grabs an EDID from disk at `tests/assets/`
#[tracing::instrument]
pub(crate) fn edid_by_filename(name: &str) -> Vec<u8> {
    let path =
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(name);
    let s = std::fs::read_to_string(path)
        .unwrap()
        .replace([' ', '\n'], "");
    hex::decode(s.trim()).unwrap()
}
