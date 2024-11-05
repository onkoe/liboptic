/// Info about the extensions following this EDID.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ExtensionInfo {
    /// The number of extension blocks (including optional block map/s) that
    /// follow the base EDID.
    ///
    /// Limited up to 255, as indicated by the type.
    pub flag: u8,

    /// Some value indicating that the entire EDID's checksum is 0x00.
    pub checksum: u8,
}
