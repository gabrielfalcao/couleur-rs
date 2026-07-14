# couleur-rs


```rust
use couleur_rs::{RGBColor, RGBValue, Result};
use std::cmp::{max, min};

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

assert_eq!(
        lightest_pink.get_adobe_complementary().to_hex_string(),
        ""
        )
```
