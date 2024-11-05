/// Identifies the display product.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct VendorProductId {
    manufacturer_name: [u8; 2],
    product_code: [u8; 2],
    serial_number: [u8; 4],
    /// note: this can either be:
    ///     (a). byte 1 is week of manufacture, byte 2 is the year of manufacture, or...
    ///     (b). byte 1 is 0xFF, byte 2 is the model's release year
    time: [u8; 2],
}
