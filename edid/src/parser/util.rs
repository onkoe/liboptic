#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

/// Grabs an EDID from disk at `tests/assets/`
#[cfg(test)]
pub(crate) fn edid_by_filename(name: &str) -> alloc::vec::Vec<u8> {
    let path =
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(name);
    let s = std::fs::read_to_string(path).unwrap();
    hex::decode(s.trim()).unwrap()
}

/// Starts the tracing subscriber.
#[cfg(test)]
pub(crate) fn logger() {
    _ = tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}
