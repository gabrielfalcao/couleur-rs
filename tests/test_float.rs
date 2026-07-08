use couleur::{Error, Result};
use couleur::{FloatMetadata, leading_zeros_fractional};
use iocore::Path;
use k9::assert_equal;
use std::cmp::{max, min};
#[test]
fn test_to_string() -> Result<()> {
    let val = 127.00141516f64;
    let fm = FloatMetadata::new(val);

    assert_equal!(val.to_string(), "127.00141516");
    assert_equal!(fm.to_string(), "127.00141516");

    Ok(())
}

#[test]
fn test_to_parts() -> Result<()> {
    let fm = FloatMetadata::new(127.00141516f64);
    let json = serde_json::to_string_pretty(&fm).unwrap();
    let path = Path::new("float_metadata.json");
    path.write(&json.as_bytes())?;
    // assert_equal!(fm.value, f64::MAX);
    // assert_equal!(fm.negative, bool::MAX);
    // assert_equal!(fm.round, i32::MAX);
    // assert_equal!(fm.fraction, i32::MAX);
    // assert_equal!(fm.leading_zeros_fractional, i32::MAX);
    Ok(())
}
