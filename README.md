# couleur-rs

parse, print, manipulate and apply contrast to RGB colors

```rust
let dark_pink = "#C32454".parse::<RGBColor>()?;
let darkest_pink = "#831C5D".parse::<RGBColor>()?;
let light_pink = "#FCA790".parse::<RGBColor>()?;
let lightest_pink = "#FDCBB0".parse::<RGBColor>()?;
assert_eq!(
    dark_pink.to_triple(),
    (
        RGBValue::from_u8(0xC3)?,
        RGBValue::from_u8(0x24)?,
        RGBValue::from_u8(0x54)?
    )
);
assert_eq!(lightest_pink.get_adobe_complementary().to_hex_string(), "#B0E2FD");
assert_eq!(lightest_pink.get_accessible_contrast().to_hex_string(), "#000000");
assert_eq!(lightest_pink.get_binary_contrast().to_hex_string(), "#000000");
assert_eq!(lightest_pink.get_msb_invert_contrast().to_hex_string(), "#7D4B30");
assert_eq!(darkest_pink.get_adobe_complementary().to_hex_string(), "#1C8342");
assert_eq!(darkest_pink.get_accessible_contrast().to_hex_string(), "#000000");
assert_eq!(darkest_pink.get_binary_contrast().to_hex_string(), "#FFFFFF");
assert_eq!(darkest_pink.get_msb_invert_contrast().to_hex_string(), "#039CDD");
```
