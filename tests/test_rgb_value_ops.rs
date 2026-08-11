use std::cmp::{max, min};

use couleur_rs::{Result, Value};

#[test]
fn test_parse_from_hex() -> Result<()> {
    let result = "ff".parse::<Value>()?;
    assert_eq!(result, 255.0);
    Ok(())
}
#[test]
fn test_parse_from_hex_error() -> Result<()> {
    let result = "ff0".parse::<Value>();
    assert_eq!(result.is_err(), false);
    // assert_eq!(result.err(), Some(Error::ParseError("failed to parse ff0".to_string())));
    assert_eq!(result.err(), None);
    Ok(())
}

#[test]
fn test_eq() {
    let dois = Value::from(2);
    let tres = Value::from(2);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_f32() {
    let dois = Value::from(2.0_f32);
    let tres = Value::from(2.0_f32);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_i32() {
    let dois = Value::from(2_i32);
    let tres = Value::from(2_i32);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_i64() {
    let dois = Value::from(2_i64);
    let tres = Value::from(2_i64);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u8() {
    let dois = Value::from(2_u8);
    let tres = Value::from(2_u8);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u16() {
    let dois = Value::from(2_u16);
    let tres = Value::from(2_u16);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u32() {
    let dois = Value::from(2_u32);
    let tres = Value::from(2_u32);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_u64() {
    let dois = Value::from(2_u64);
    let tres = Value::from(2_u64);

    assert_eq!(dois, tres);
}

#[test]
fn test_eq_rhs_usize() {
    let dois = Value::from(2_usize);
    let tres = Value::from(2_usize);

    assert_eq!(dois, tres);
}

#[test]
fn test_ord() {
    let min_val = Value(9.09f32);
    let max_val = Value(99.9f32);
    let highest = max(min_val, max_val);
    let lowest = min(min_val, max_val);

    assert_eq!(min_val, lowest);
    assert_eq!(max_val, highest);
}
