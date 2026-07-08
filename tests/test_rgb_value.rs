use couleur::RGBValue;
use couleur::Result;
use k9::assert_equal;

#[test]
fn test_parse_from_hex() -> Result<()> {
    let result = "ff".parse::<RGBValue>()?;
    assert_equal!(result, 255.0);
    assert_equal!(result.to_hex_rgb(), "#FFB08B");
    assert_equal!(result.value(), 255.0);
    assert_equal!(result.round(), 255.0);
    assert_equal!(result.fract(), 0.0);
    assert_equal!(result.copysign(&1.0), 255.0);
    assert_equal!(result.to_u8()?, 255);
    assert_equal!(result.into_u8(), 255);
    assert_equal!(RGBValue::from_u8(250u8), 250);
    assert_equal!(result.leading_zeros_exp(), 0);
    assert_equal!(result.leading_zeros_fractional(), 0);
    assert_equal!(result.float_metadata(), (false, 255, 0, 0, 0));
    // assert_equal!(result.float_metadata(), (250, 0, 0, 0, 0));
    assert_equal!(result.to_hex_rgb(), "#FFB08B");
    Ok(())
}
