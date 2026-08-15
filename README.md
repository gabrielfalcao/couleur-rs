# couleur-rs

parse, print, manipulate and apply contrast to RGB colors

## Naming

The name *"couleur"* means *"color"* in french and is the [name of the
python library I wrote several years
ago(https://pypi.org/project/couleur/), same purpose but less powerful
in terms of features in comparison with this rust crate.

The crate name `couleur` has already been taking, hence the `-rs`
suffix in this crate' name.

## Keeping the python and rust libraries in sync

I plan on eventually either rewrite it part counterpart to match the
features, keeping that python package written in pure native code, or
make that package a binding to the rust crate via
[maturin](https://crates.io/crates/maturin) and
[pyo3](https://crates.io/crates/pyo3).


## Basic Example Usage of this crate

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
