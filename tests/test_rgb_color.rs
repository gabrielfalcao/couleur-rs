use couleur_rs::{Algorithm, Layer, Color, Value, Reset, Result, Wrap};
use k9::assert_equal;
use std::cmp::{max, min};

#[test]
fn test() -> Result<()> {
    // use crate::{Color, Value, Result};
    // use std::cmp::{max, min};

    let dark_pink = "#C32454".parse::<Color>()?;
    let darkest_pink = "#831C5D".parse::<Color>()?;
    let light_pink = "#FCA790".parse::<Color>()?;
    let lightest_pink = "#FDCBB0".parse::<Color>()?;

    assert_eq!(
        dark_pink.to_triple(),
        [
            Value::from_u8(0xC3)?,
            Value::from_u8(0x24)?,
            Value::from_u8(0x54)?
        ]
    );

    assert_eq!(
        lightest_pink.get_adobe_complementary().to_hex_string(),
        "#B0E2FD"
    );
    assert_eq!(
        lightest_pink.get_accessible_contrast().to_hex_string(),
        "#000000"
    );
    assert_eq!(
        lightest_pink.get_binary_contrast().to_hex_string(),
        "#000000"
    );
    assert_eq!(
        lightest_pink.get_msb_invert_contrast().to_hex_string(),
        "#7D4B30"
    );

    assert_eq!(
        darkest_pink.get_adobe_complementary().to_hex_string(),
        "#1C8342"
    );
    assert_eq!(
        darkest_pink.get_accessible_contrast().to_hex_string(),
        "#000000"
    );
    assert_eq!(
        darkest_pink.get_binary_contrast().to_hex_string(),
        "#FFFFFF"
    );
    assert_eq!(
        darkest_pink.get_msb_invert_contrast().to_hex_string(),
        "#039CDD"
    );
    Ok(())
}

#[test]
fn test_parse() -> Result<()> {
    let dark_pink = "#C32454".parse::<Color>()?;
    let darkest_pink = "#831C5D".parse::<Color>()?;
    let light_pink = "#FCA790".parse::<Color>()?;
    let lightest_pink = "#FDCBB0".parse::<Color>()?;

    assert_eq!(
        dark_pink.to_triple(),
        [
            Value::from_u8(0xC3)?,
            Value::from_u8(0x24)?,
            Value::from_u8(0x54)?
        ]
    );

    Ok(())
}
#[test]
fn test_parse_and_get_accessible_contrast() -> Result<()> {
    // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
    // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
    // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
    // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
    // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
    let lightest: Color = "#8FF8E2".parse()?;
    let darkest: Color = "#0B5E65".parse()?;
    assert_equal!(
        lightest.get_accessible_contrast(),
        Color::from_triple(0.into(), 0.into(), 0.into())
    );
    assert_equal!(
        darkest.get_accessible_contrast(),
        Color::from_triple(255.into(), 255.into(), 255.into())
    );
    Ok(())
}
#[test]
fn test_parse_and_get_binary_contrast() -> Result<()> {
    // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
    // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
    // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
    // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
    // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
    let lightest: Color = "#8FF8E2".parse()?;
    let darkest: Color = "#0B5E65".parse()?;
    assert_equal!(
        lightest.get_binary_contrast(),
        Color::from_triple(0.into(), 0.into(), 0.into())
    );
    assert_equal!(
        darkest.get_binary_contrast(),
        Color::from_triple(255.into(), 255.into(), 255.into())
    );
    Ok(())
}

#[test]
fn test_parse_and_get_adobe_complementary() -> Result<()> {
    // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
    // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
    // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
    // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
    // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
    let lightest: Color = "#8FF8E2".parse()?;
    let darkest: Color = "#0B5E65".parse()?;
    assert_equal!(
        lightest.get_adobe_complementary(),
        Color::from_triple(248.into(), 143.into(), 165.into())
    );
    assert_equal!(
        darkest.get_adobe_complementary(),
        Color::from_triple(101.into(), 18.into(), 11.into())
    );
    Ok(())
}
#[test]
fn test_parse_and_get_msb_invert_contrast() -> Result<()> {
    // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
    // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
    // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
    // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
    // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
    let lightest: Color = "#8FF8E2".parse()?;
    let darkest: Color = "#0B5E65".parse()?;
    assert_equal!(
        lightest.get_msb_invert_contrast(),
        Color::from_triple(15.into(), 120.into(), 98.into())
    );
    assert_equal!(
        darkest.get_msb_invert_contrast(),
        Color::from_triple(139.into(), 222.into(), 229.into())
    );
    Ok(())
}
#[test]
fn test_wrap_ansi() -> Result<()> {
    let color: Color = "#FFCC00".parse()?;
    let text = "test 123";
    let fg = color.wrap_ansi(
        text,
        Some(Layer::FG),
        true,
        Some(Wrap::Around),
        Some(Reset::After),
        Some(Algorithm::Web),
    );
    assert_equal!(
        format!("{fg}"),
        "\u{1b}[1;38;2;255;204;0mtest 123\u{1b}[1;38;2;255;204;0m\u{1b}[0m"
    );

    Ok(())
}
