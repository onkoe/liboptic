<!-- cargo-rdme start -->

# `liboptic_edid`

A library crate to parse EDID information.

Currently, this uses EDID v1.4 and does not account for any incompatbilities with earlier versions. However, after running against around 100k EDIDs, there are only about 5% failing to parse, and no panics.

## Usage

The only significant type in the library is `Edid`. Call `Edid::new()` with your EDID in bytes to get it parsed!

```rust
use liboptic_edid::Edid;

// grab the edid file from disk
let data = std::fs::read("tests/assets/dell_s2417dg.raw.input")?;

// and load it into the parser
let parsed_edid = Edid::new(&data)?;
assert_eq!(parsed_edid.checksum, 0x51);
```

## Compatibility

This crate is `#![no_std]` but still depends on `alloc` while `pisserror` does. When I get around to fixing that, that requirement will be dropped. :)

<!-- cargo-rdme end -->
