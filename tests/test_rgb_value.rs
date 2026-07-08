use couleur::RGBValue;
use couleur::Result;
use k9::assert_equal;

#[test]
fn test_parse_from_hex() -> Result<()> {
    let result = "ff".parse::<RGBValue>()?;
    assert_equal!(result, 255.0);
    Ok(())
}
