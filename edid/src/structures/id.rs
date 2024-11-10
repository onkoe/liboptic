use arrayvec::ArrayString;

/// Identifies the display product.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VendorProductId {
    /// The name of the display's manufacturer.
    pub manufacturer_name: Manufacturer,

    /// The manufacturer-unique identifier for this display.
    pub product_code: u16,

    /// The serial number of this display, if present.
    pub serial_number: Option<u32>,

    /// Info about when the display came from.
    pub date: Date,
}

/// The display's manufacturer.
///
/// This differentiates between vendors with PNP IDs and those who are
/// non-compliant.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Manufacturer {
    Name(ArrayString<{ pnpid::MAX_LEN }>),
    Id(ArrayString<3>),
}

/// Info about the date the display came from, either in terms of
/// manufacturing, or when it was released.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Date {
    /// The approximate date the display was manufactured, down to the week,
    /// if given by the manufacturer.
    Manufacture {
        /// A week value from 1-54, if given.
        week: Option<u8>,
        /// The year this display was manufactured in.
        year: u16,
    },

    /// The year the display model was released.
    ///
    /// Note that this is not the display's year of manufacture.
    ModelYear(u16),
}
