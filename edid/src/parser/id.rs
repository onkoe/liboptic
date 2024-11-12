//! `id`: the second step in parsing.
//!
//! This contains vendor and product information.

use arrayvec::ArrayString;
use bitvec::{field::BitField, order::Msb0, view::BitView};
use id::Manufacturer;

use crate::structures::{self, id::VendorProductId};

use crate::prelude::internal::*;

/// Parses out the `VendorProductId` given the raw input.
#[tracing::instrument(skip_all)]
pub(super) fn parse(input: &[u8]) -> Result<VendorProductId, EdidError> {
    // the first two bytes are the manufacturer name
    let manufacturer_name = vendor(&mut [input[0x08], input[0x09]])?;

    // the next two make a unique hex number indicating which display model
    // we've got.
    //
    // this is from the manufacturer. no text conversion.
    let product_code: u16 = bytemuck::must_cast([input[0x0A], input[0x0B]]);

    // same thing here for the serial number.
    //
    /* TODO:
    Note for Table 3.7: The EDID structure version 1, revision 1 (and newer) offers a way to represent the
    serial number of the monitor as an ASCII string in a separate descriptor block. Refer to section 3.10.3
    Display Descriptors for an alternative method of defining a serial number.
    */
    let serial_section = [input[0x0C], input[0x0D], input[0x0E], input[0x0F]];
    let serial: u32 = bytemuck::must_cast(serial_section);

    // if the serial is zero
    let serial_number = if serial == 0 { None } else { Some(serial) };

    /* two more bytes for week/year!
        - week:
            - 0x00: no week specified
            - 0xFF: year byte indicates model year
            - others: week from 1-54th week of year
        - year
    */
    let week_byte = input[0x10];

    // the week is optional. 0x00 means "none"
    let week = if week_byte == 0x00 {
        None
    } else {
        Some(week_byte)
    };

    // we add "1990" to the year to restore it from a u8
    let year = (input[0x11] as u16) + 1990;

    // determine if it's a model release or manufacturing date
    let date = if week == Some(0xFF) {
        // it's the model year!
        structures::id::Date::ModelYear(year)
    } else {
        // it's the year of manufacture.
        structures::id::Date::Manufacture { week, year }
    };

    // construct the info!
    Ok(VendorProductId {
        manufacturer_name,
        product_code,
        serial_number,
        date,
    })
}

/// Gets the vendor (company) name from the given input.
///
/// The input should always be exactly two elements long, containing three
/// 5-bit ASCII values.
#[tracing::instrument]
fn vendor(input: &mut [u8; 2]) -> Result<Manufacturer, EdidError> {
    // let's grab the PNP ID.
    let bits = input[0..=1].view_bits_mut::<Msb0>();

    // make an array of "u5" (chunks of five bits) casted to u8
    let mut bits = bits[1..16].chunks(5);
    let arr = (|| {
        Some([
            bits.next()?.load_be::<u8>(),
            bits.next()?.load_be::<u8>(),
            bits.next()?.load_be::<u8>(),
        ])
    })()
    .ok_or_else(|| {
        tracing::error!("Failed to load required bits for PNP ID parsing. Just sending an ID...");
        EdidError::IdBadValues(*input)
    })?;
    tracing::trace!("Found all three `u5` values. ({:#?})", &arr);

    // convert it into rust (ascii) chars
    let chars = convert_5bit_ascii_slice(arr)?;
    tracing::trace!("Successfully converted into a char array. ({:#?})", &chars);

    // we'll make an arrayvec::ArrayString. that can become a &str
    let mut string: ArrayString<3> = ArrayString::new_const();
    string.push(chars[0]);
    string.push(chars[1]);
    string.push(chars[2]);
    tracing::trace!("Created ArrayString. (`{}`)", string);

    // let's try to find the its name from their pnp id
    Ok(match pnpid::company_from_pnp_id(string.as_str()) {
        Some(name) => {
            tracing::debug!("Got a company name! (`{name}`)");

            // finally, return the company name
            let n = ArrayString::<{ pnpid::MAX_LEN }>::from(name).map_err(|e| {
                tracing::error!("Couldn't fit company name into ArrayString! (err: {e})");
                EdidError::ArrayStringError
            })?;

            Manufacturer::Name(n)
        }
        None => {
            tracing::warn!("Failed to find company name from the EDID's PNP ID: `{string}`.");
            Manufacturer::Id(string)
        }
    })
}

/// Converts three u5 ASCII values into Rust `char`s.
#[tracing::instrument]
fn convert_5bit_ascii_slice(codes: [u8; 3]) -> Result<[char; 3], EdidError> {
    Ok([
        convert_5bit_ascii(codes[0])?,
        convert_5bit_ascii(codes[1])?,
        convert_5bit_ascii(codes[2])?,
    ])
}

/// Converts the compressed 5-bit ASCII code into a standard Rust character.
#[tracing::instrument]
fn convert_5bit_ascii(code: u8) -> Result<char, EdidError> {
    const CODES: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    // we don't index by zero
    if code == 0 {
        tracing::error!("Attempted to get the zeroth letter in ASCII, but this isn't correct.");
        return Err(EdidError::IdNoZeroesAllowed);
    }

    if let Some(c) = CODES.get((code - 1) as usize) {
        tracing::debug!("Got a char! (code: `{code}`, char: `{c}`)");
        Ok(*c)
    } else {
        tracing::error!("Passed in an invalid character identifier (`{code}`).");
        Err(EdidError::CharOutOfBounds(code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{prelude::internal::logger, structures::id::Date};

    #[test]
    fn dell_s2417dg_id() {
        logger();
        let input = crate::prelude::internal::raw_edid_by_filename("dell_s2417dg.raw.input");
        let vendor_product_info = super::parse(&input).unwrap();

        // check the various properties

        // date
        let Date::Manufacture { week, year } = vendor_product_info.date else {
            panic!("found wrong date kind");
        };
        assert_eq!(week.unwrap(), 28);
        assert_eq!(year, 2018);

        // vendor
        assert_eq!(
            vendor_product_info.manufacturer_name,
            Manufacturer::Name(ArrayString::from("Dell Inc.").unwrap())
        );

        // model
        assert_eq!(vendor_product_info.product_code, 41191);

        // serial
        assert_eq!(vendor_product_info.serial_number, Some(1));
    }

    #[test]
    fn that_guys_laptop() {
        logger();
        let input = crate::prelude::internal::edid_by_filename("1.input");
        let vendor_product_info = super::parse(&input).unwrap();

        // check the various properties

        // date
        let Date::Manufacture { week, year } = vendor_product_info.date else {
            panic!("found wrong date kind");
        };
        assert!(week.is_none());
        assert_eq!(year, 2012);

        // vendor
        assert_eq!(
            vendor_product_info.manufacturer_name,
            Manufacturer::Name(ArrayString::from("DO NOT USE - AUO").unwrap())
        );

        // model
        assert_eq!(vendor_product_info.product_code, 8237);

        // serial
        assert!(vendor_product_info.serial_number.is_none());
    }

    #[test]
    fn test_5bit() {
        // these are the end points, so they should work.
        let a = convert_5bit_ascii(0b00001).unwrap();
        let z = convert_5bit_ascii(0b11010).unwrap();

        assert_eq!(a, 'A');
        assert_eq!(z, 'Z');

        // we're one-indexed, not zero-indexed. so 0x0 should fail
        let zero = convert_5bit_ascii(0b00000);
        assert!(zero.is_err());

        // we'll also check every letter, just to be safe.
        let chars = 'A'..='Z';
        let numbers = 0b00001..=0b11010;

        for (code, expected_char) in numbers.zip(chars) {
            let actual = convert_5bit_ascii(code).unwrap();
            assert_eq!(expected_char, actual);
        }
    }

    #[test]
    fn test_5bit_dell() {
        let arr: [u8; 3] = [
            0x04, // 'D'
            0x05, // 'E',
            0x0C, // 'L',
        ];

        assert_eq!(convert_5bit_ascii_slice(arr).unwrap(), ['D', 'E', 'L']);
    }
}
