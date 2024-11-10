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

    let failed = format!(
        "{}{}",
        parsed_edids
            .iter()
            .filter(|pe| pe.parsed_edid.is_err())
            .count()
            .to_string()
            .bright_red(),
        " failed".red()
    );

    let panicked = format!(
        "{}{}",
        unsafe { PANICKED }.to_string().black().on_red(),
        " panicked".black().on_red()
    );

    // afterwards, we can list the results
    if LIST_ERRS {
        for edid in parsed_edids {
            if let Err(e) = edid.parsed_edid {
                println!("{} {}\n", "[ERR]".black().on_red(), e.to_string().red());
            }
        }
    }

    // and print the strings
    println!("({passed}/{failed}/{panicked})");

    Ok(())
}

/// Gets and parses all EDIDs.
fn parse_all() -> Vec<Entry> {
    let path = canonicalize(PathBuf::from(PATH)).expect("canonicalize");
    println!("checking at path `{}`", path.display());

    // run on each entry in both digital and analog
    let timer = Instant::now();
    let paths = [path.join("Digital"), path.join("Analog")]
        .par_iter()
        .flat_map(|p| {
            let inner: Vec<_> = jwalk::WalkDir::new(p)
                .into_iter()
                .flat_map(|entry| {
                    if let Ok(entry) = entry {
                        Some(entry.path())
                    } else {
                        None
                    }
                })
                .map(run) // run the logic
                .collect();
            inner
        })
        .collect::<Vec<Option<_>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

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

    paths
}

/// An EDID we parsed.
struct Entry {
    parsed_edid: Result<Edid, EdidError>,
    raw_edid: Vec<u8>,
    path: PathBuf,
}

/// contains logic to run on both digital and analog displays.
fn run<P: AsRef<Path>>(entry_path: P) -> Option<Entry> {
    let entry_path = entry_path.as_ref();

    if let Some(edid) = edid_by_filename(entry_path) {
        return Some(Entry {
            parsed_edid: Edid::new(edid.clone()),
            raw_edid: edid,
            path: entry_path.into(),
        });

        /*
        // parse it and return the results
        let parse_result = std::panic::catch_unwind(|| Edid::new(edid.clone()));

        // report any panic
        match parse_result {
            Ok(parsed_edid) => {
                return Some(Entry {
                    parsed_edid,
                    raw_edid: edid,
                    path: entry_path.into(),
                });
            }
            Err(_) => {
                tracing::error!("PANICKED ON FILE! (path: `{}`)", entry_path.display());

                // SAFETY: who cares
                unsafe { PANICKED += 1 };

                return None;
            }
        }
        */
    }

    None
}

/// Grabs an EDID from disk at the given path.
#[tracing::instrument]
pub(crate) fn edid_by_filename(path: &Path) -> Option<Vec<u8>> {
    let path = PathBuf::from(path);
    let s = read_to_string(&path).ok()?;

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

    hr.ok()
}
