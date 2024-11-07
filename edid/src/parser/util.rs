#[cfg(test)]
extern crate alloc;
#[cfg(test)]
extern crate std;

/// Grabs an EDID from disk at `tests/assets/`
#[cfg(test)]
#[tracing::instrument]
pub(crate) fn edid_by_filename(name: &str) -> alloc::vec::Vec<u8> {
    if name.contains(".info") {
        tracing::warn!(
            "You're probably passing in a `.info` file for the test.\
        However, you probably meant to use a `.input` file instead!"
        );
    }

    let path =
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(name);
    let s = std::fs::read_to_string(path)
        .unwrap()
        .replace(' ', "")
        .replace('\n', "");
    hex::decode(s.trim()).unwrap()
}

/// Grabs a raw (not encoded) EDID from disk at `tests/assets/`
#[cfg(test)]
#[tracing::instrument]
pub(crate) fn raw_edid_by_filename(name: &str) -> alloc::vec::Vec<u8> {
    let path =
        std::path::PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/assets")).join(name);

    std::fs::read(path).unwrap()
}

/// Starts the tracing subscriber.
#[cfg(test)]
#[tracing::instrument]
pub(crate) fn logger() {
    _ = tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
}
