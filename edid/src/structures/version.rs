/// Contains info about which version + revision of the standard this structure
/// expects.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EdidVersion {
    /// Should be 0x01 for v1.4.
    version: u8,
    /// Should be 0x04 for v1.4.
    revision: u8,
}
