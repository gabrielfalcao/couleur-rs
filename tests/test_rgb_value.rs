use couleur_rs::{Color, Value, Result};
use k9::assert_equal;
use std::cmp::{max, min};

#[test]
fn test_value_from_u8() -> Result<()> {
    let dark_pink = "#C32454".parse::<Color>()?;

    assert_eq!(Value::from_u8(0xc3u8)?, Value(195.0f32));
    assert_eq!(Value::from_u8(0xc3u8)?, Value::new(195.0f32)?);

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
    Ok(())
}
#[test]
fn test_parse_from_hex() -> Result<()> {
    let result = "ff".parse::<Value>()?;
    assert_equal!(result, 255.0);
    assert_equal!(result.value(), 255.0);
    // assert_equal!(result.round(), 255.0);
    // assert_equal!(result.fract(), 0.0);
    // assert_equal!(result.copysign(&1.0), 255.0);
    // assert_equal!(result.to_u8()?, 255);
    // assert_equal!(result.into_u8(), 255);
    // assert_equal!(Value::from_u8(250u8)?, 250);
    // assert_equal!(result.leading_zeros_exp(), 0);
    // assert_equal!(result.leading_zeros_fractional(), 0);
    // // assert_equal!(result.float_metadata(), (0, 0, 0, 0, 0));
    // assert_equal!(result.float_metadata(), (false, 255, 0, 0, 0));
    Ok(())
}
