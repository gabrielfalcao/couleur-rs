use couleur::RGBValue;
use couleur::Result;
use k9::assert_equal;
use std::cmp::{max, min};

#[test]
fn test_parse_from_hex() -> Result<()> {
    let result = "ff".parse::<RGBValue>()?;
    assert_eq!(result, 255.0);
    Ok(())
}

#[test]
fn test_eq() {
    let dois = RGBValue::from(2);
    let tres = RGBValue::from(2);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_f32() {
    let dois = RGBValue::from(2.0_f32);
    let tres = RGBValue::from(2.0_f32);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_i32() {
    let dois = RGBValue::from(2_i32);
    let tres = RGBValue::from(2_i32);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_i64() {
    let dois = RGBValue::from(2_i64);
    let tres = RGBValue::from(2_i64);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u8() {
    let dois = RGBValue::from(2_u8);
    let tres = RGBValue::from(2_u8);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u16() {
    let dois = RGBValue::from(2_u16);
    let tres = RGBValue::from(2_u16);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u32() {
    let dois = RGBValue::from(2_u32);
    let tres = RGBValue::from(2_u32);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u64() {
    let dois = RGBValue::from(2_u64);
    let tres = RGBValue::from(2_u64);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_usize() {
    let dois = RGBValue::from(2_usize);
    let tres = RGBValue::from(2_usize);

    assert_eq!(dois, tres);
}

#[test]
fn test_ord() {
    let min_val = RGBValue(9.09f32);
    let max_val = RGBValue(99.9f32);
    let highest = max(min_val, max_val);
    let lowest = min(min_val, max_val);

    assert_equal!(min_val, lowest);
    assert_equal!(max_val, highest);
}
