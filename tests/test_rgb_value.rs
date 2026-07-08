use couleur::RGBValue;
use couleur::Result;
use k9::assert_equal;

#[test]
fn test_parse_from_hex() -> Result<()> {
    let result = "ff".parse::<RGBValue>()?;
    assert_equal!(result, 255.0);
    assert_equal!(result.to_hex_rgb(), "#90B08B");
    Ok(())
}
