//! Loads an EDID file from the Linux `sysfs` and parses it.

#![cfg(target_os = "linux")]

fn main() -> Result<(), Box<dyn core::error::Error>> {
    // let's parse an edid file from disk! on linux, each monitor has one
    // located on the `sysfs`!
    //
    // here's mine! you can find yours by `cat`-ing each `card<N-STUFF>` and
    // looking for files with gibberish in them .
    let file_path = "/sys/class/drm/card1-DP-3/edid";

    // now, load the file into a slice. make sure to use a method that returns
    // raw bytes! a string won't work for this.
    let bytes = std::fs::read(file_path)?;

    // finally, just feed it to the parser
    let parsed_edid = liboptic_edid::Edid::new(&bytes)?;

    // here's the checksum!
    println!("Checksum: {:x}", parsed_edid.checksum);

    Ok(())
}
