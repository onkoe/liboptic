//! Tries to parse each and every EDID in the `linuxhw/edid` repo.
//!
//! ```cargo
//! [package]
//! name = "crater_run"
//!
//! [dependencies]
//! liboptic_edid = { path = "../" } # the uhh. crate
//! walkdir = "2"
//! jwalk = "0.5" # using this instead for rayon support.
//! anyhow = "1"
//! hex = "0.4"
//! rayon = "1.10"
//! colored = "2"
//! tracing-subscriber = { version = "0.3.18", features = ["alloc", "fmt", "env-filter"] }
//! tracing = "0.1.40"
//! ```

use std::fs::{canonicalize, read_to_string};
use std::path::{Path, PathBuf};
use std::time::Instant;

use colored::*;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt as _, util::SubscriberInitExt, EnvFilter,
};

use liboptic_edid::{error::EdidError, Edid};

static mut PANICKED: u32 = 0_u32;

static mut SKIPPED: u32 = 0_u32;

const PATH: &str = "/home/barrett/Downloads/linuxhw_edid_repo";
const LIST_ERRS: bool = false;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::WARN)
        .init();

    // parse all of them
    println!("parsing all edids... (this may take a sec)");
    let parsed_edids = parse_all();

    let passed = format!(
        "{}{}",
        parsed_edids
            .iter()
            .flat_map(|pe| &pe.parsed_edid)
            .count()
            .to_string()
            .bright_green(),
        " passed".green()
    );

    let expected_fails = format!(
        "{}{}{}",
        parsed_edids
            .iter()
            .filter(|pe| pe.parsed_edid.is_err() && matches!(pe.should_fail, ExpectedFailure::Yes))
            .count()
            .to_string()
            .bright_red()
            .on_black(),
        " failed... ".red().on_black(),
        "expectedly".green().on_black()
    );

    let panicked = format!(
        "{}{}",
        unsafe { PANICKED }.to_string().black().on_red(),
        " panicked".black().on_red()
    );

    // afterwards, we can list the results
    if LIST_ERRS {
        for edid in parsed_edids {
            if let Err(ref e) = edid.parsed_edid {
                println!("{} {}\n", "[ERR]".black().on_red(), e.to_string().red());
            }
        }
    }

    // and print the strings
    println!("({passed}/{expected_fails}/{panicked})");

    Ok(())
}

/// Gets and parses all EDIDs.
fn parse_all() -> Vec<Entry> {
    let path = canonicalize(PathBuf::from(PATH)).expect("canonicalize");
    println!("checking at path `{}`", path.display());

    // run on each entry in both digital and analog
    let timer = Instant::now();
    let dirs = [path.join("Digital"), path.join("Analog")];

    let mut runs = Vec::<Entry>::new();
    for dir in dirs {
        for entry in walkdir::WalkDir::new(dir) {
            if let Ok(entry) = entry {
                if let Some(res) = run(entry.path()) {
                    runs.push(res.clone());
                    if res.parsed_edid.is_err() {
                        println!(
                            "err on {}th run! (path: {})",
                            runs.len(),
                            res.path.display()
                        );
                        hex(&res.raw_edid);
                    }

                    if matches!(res.should_fail, ExpectedFailure::Yes) {
                        // expected to fail. all good!
                        assert!(
                            res.parsed_edid.is_err(),
                            "expected fail should fail, but was ok: {res:#?}"
                        );
                    } else {
                        res.parsed_edid.expect("found an unexpected failure.");
                    }
                }
            }
        }
    }

    // say how long it took
    println!(
        "{}\n",
        format!(
            "Completed in {:.2} seconds.",
            Instant::now().duration_since(timer).as_secs_f32()
        )
        .white()
        .on_green()
    );

    runs
}

/// An EDID we parsed.
#[derive(Clone, Debug)]
struct Entry {
    parsed_edid: Result<Edid, EdidError>,
    raw_edid: Vec<u8>,
    path: PathBuf,
    should_fail: ExpectedFailure,
}

/// prints a string of hex bytes, causing a given byte array to be more
/// readable.
fn hex(input: &[u8]) {
    // print column numbers
    print!("{}", "[--]: ".blue());
    for c in 0x00..0x10 {
        print!("{}", format!("{c:02x} ").blue())
    }
    println!();

    for (chunk_num, chunk) in input.chunks(16).enumerate() {
        // print the chunk number before the actual data
        print!("{}", &format!("[{chunk_num:02x}]: ").red());

        for byte in chunk {
            print!("{}", format!("{:02x} ", byte).as_str());
        }
        println!();
    }
}

/// contains logic to run on both digital and analog displays.
fn run<P: AsRef<Path>>(entry_path: P) -> Option<Entry> {
    let entry_path = entry_path.as_ref();

    if let Some((bytes, should_fail)) = edid_by_filename(entry_path) {
        // return Some(Entry {
        //     parsed_edid: Edid::new(edid.clone()),
        //     raw_edid: bytes,
        //     path: entry_path.into(),
        //     should_fail,
        // });

        // parse it and return the results
        let parse_result = std::panic::catch_unwind(|| Edid::new(bytes.clone()));

        // report any panic
        match parse_result {
            Ok(parsed_edid) => {
                return Some(Entry {
                    parsed_edid,
                    raw_edid: bytes,
                    path: entry_path.into(),
                    should_fail,
                });
            }
            Err(_) => {
                tracing::error!("PANICKED ON FILE! (path: `{}`)", entry_path.display());

                // SAFETY: who cares
                unsafe { PANICKED += 1 };

                return None;
            }
        }
    }

    None
}

/// Grabs an EDID from disk at the given path.
#[tracing::instrument]
pub(crate) fn edid_by_filename(path: &Path) -> Option<(Vec<u8>, ExpectedFailure)> {
    let path = PathBuf::from(path);
    let s = read_to_string(&path).ok()?;

    let mut expected_failure = ExpectedFailure::No;

    // skip any that we don't expect to work
    {
        if s.contains("Vendor & Product Identification: Manufacturer name field contains garbage.")
        {
            expected_failure = ExpectedFailure::Yes;
        }

        if s.contains("Display Range Limits: Byte 10 is ") && s.contains(" instead of 0x0a.") {
            expected_failure = ExpectedFailure::Yes;
        }

        if s.contains("Display Range Limits: Unknown range class") {
            expected_failure = ExpectedFailure::Yes;
        }
    }

    // skip any non-edid files
    if !s.contains("edid-decode (hex):") {
        return None;
    }

    // split on the dashes. only grab the first part (the given input)
    let v: Vec<&str> = s.split("----------------").collect();
    let s = v.first()?.trim();

    // remove the weird title they added to the files
    let s = s.replace("edid-decode (hex):", "");

    // remove all whitespace
    let s = s.replace([' ', '\n', '\r'], "");

    let hr = hex::decode(&s).inspect_err(|e| {
        tracing::error!("couldn't turn into hex! (err: {e}) for the following hex:");
        eprintln!("{s}")
    });

    hr.ok().map(|hr| (hr, expected_failure))
}

#[derive(Clone, Debug)]
enum ExpectedFailure {
    Yes,
    No,
}
