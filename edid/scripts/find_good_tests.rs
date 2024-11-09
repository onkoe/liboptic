#!/usr/bin/env rust-script
//!
//! Looks through folders of files with EDIDs in hex format to find any that
//! contain a specific byte pattern.
//!
//! This can assist when looking for test assets.
//!
//! ```cargo
//! [dependencies]
//! walkdir = "2"
//! jwalk = "0.5" # using this instead for rayon support.
//!
//! # conversion and runtime
//! anyhow = "1"
//! hex = "0.4"
//! rayon = "1.10"
//!
//! # output
//! colored = "1"
//! tracing-subscriber = { version = "0.3.18", features = ["alloc", "fmt"] }
//! tracing = "0.1.40"
//! ```

use std::fs::{canonicalize, read_to_string};
use std::path::{Path, PathBuf};
use std::time::Instant;

use colored::Colorize as _;
use rayon::prelude::*;

const PATH: &str = "/home/barrett/Downloads/linuxhw_edid_repo";

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

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
        .collect::<Vec<Option<PathBuf>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<PathBuf>>();

    // say how long it took
    println!(
        "Completed in {:.2} seconds.\n",
        Instant::now().duration_since(timer).as_secs_f32()
    );

    // print the results
    print_results(&paths);

    Ok(())
}

/// contains logic to run on both digital and analog displays.
fn run<P: AsRef<Path>>(entry_path: P) -> Option<PathBuf> {
    let entry_path = entry_path.as_ref();

    if let Some(edid) = edid_by_filename(entry_path) {
        // i want to find edids that use the 0xFB byte on one of their 18 byte
        // data blocks.
        //
        // all data block starting indices. first is a preferred timing mode,
        // so it's not included
        let _18b_data_blocks = [0x48, 0x5a, 0x6c];

        // the kind im looking for has these bytes
        let wanted = [0x00, 0x00, 0x00, 0xFF];

        // check each block at the given location
        if _18b_data_blocks
            .iter()
            .any(|b| &edid[(*b)..(*b + wanted.len())] == &wanted)
        {
            // yay its a good one
            // println!("{}", entry_path.display());
            return Some(entry_path.into());
        }
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

/// Prints the given list of results.
fn print_results(paths: &Vec<PathBuf>) {
    if paths.is_empty() {
        // print a sad message
        println!("{}", "Found no matching files!".red().on_black());
    } else {
        // list the matches
        println!("Here's a list of all matching files: ");
        for p in paths.chunks(2) {
            if let Some(p0) = p.get(0) {
                println!("{}", p0.display().to_string().white().on_black());
            }
            if let Some(p1) = p.get(1) {
                println!("{}", p1.display().to_string().white().on_bright_black());
            }
        }
        println!();

        // say how long the list is
        println!(
            "{}{}{}",
            "Found ".green().on_black(),
            paths.len().to_string().bright_green().on_black(),
            " matching files.".green().on_black()
        );
    }
}
