use couleur_rs::{FloatMetadata, Result};
use iocore::Path;

#[test]
fn test_to_string() -> Result<()> {
    let val = 127.00141516f64;
    let fm = FloatMetadata::new(val);

    assert_eq!(val.to_string(), "127.00141516");
    assert_eq!(fm.to_string(), "127.00141516");

    Ok(())
}

#[test]
fn test_to_parts() -> Result<()> {
    let fm = FloatMetadata::new(127.00141516f64);
    let json = serde_json::to_string_pretty(&fm).unwrap();
    let path = Path::new("float_metadata.json");
    path.write(&json.as_bytes())?;
    // assert_eq!(fm.value, f64::MAX);
    // assert_eq!(fm.negative, bool::MAX);
    // assert_eq!(fm.round, i32::MAX);
    // assert_eq!(fm.fraction, i32::MAX);
    // assert_eq!(fm.leading_zeros_fractional, i32::MAX);
    Ok(())
}
