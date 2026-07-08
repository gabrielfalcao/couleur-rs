use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub struct FloatMetadata<'a> {
    value: &'a str,
    negative: bool,
    round: i32,
    fraction: i32,
    leading_zeros_fractional: i32,
}
impl<'a> FloatMetadata<'a> {
    pub fn from_parts(parts: (f64, bool, i32, i32, i32)) -> FloatMetadata<'a> {
        let (value, negative, round, fraction, leading_zeros_fractional) = parts;
        let value = value.to_string();
        FloatMetadata {
            value: value.leak(),
            negative,
            round,
            fraction,
            leading_zeros_fractional,
        }
    }
    pub fn to_parts(&self) -> (f64, bool, i32, i32, i32) {
        (
            self.value.parse::<f64>().unwrap(),
            self.negative,
            self.round,
            self.fraction,
            self.leading_zeros_fractional,
        )
    }
    pub fn new(value: f64) -> FloatMetadata<'a> {
        let negative = value.round() < 0.0;

        let float_round = value.round();

        let float_fract = value.fract();
        let nonzero_fract = leading_zeros_fractional(&value);
        // dbg!(leading_fract, nonzero_fract);
        FloatMetadata::from_parts((
            value,
            negative,
            float_round as i32,
            leading_fract as i32,
            nonzero_fract as i32,
        ))
    }
}
impl std::fmt::Display for FloatMetadata<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
// pub fn leading_zeros_fractional<T: Copy + Deref<Target = f32>>(float: T) -> usize {
//     let s = float.to_string();
//     if let Some(dot_idx) = s.find('.') {
//         s[dot_idx + 1..].chars().take_while(|&c| c == '0').count()
//     } else {
//         0
//     }
// }
// pub fn leading_zeros_exp<T: Copy + Deref<Target = f32>>(float: T) -> i32 {
//     let float_fract_leading_zeroes = leading_zeros_fractional(float);
//     let float_fract = float.fract().copysign(1.0_f32);
//     let mut exp = float.fract();
//     for _ in 0..float_fract_leading_zeroes {
//         exp = exp * 10.0;
//     }
//     assert!(exp >= 0.0, "expected {exp} to be >= 0.0");
//     return exp as i32;
// }

pub fn f32_metadata<T: Copy + Deref<Target = f64>>(float: T) -> (bool, i32, i32, i32, usize) {
    let value = RGBValue(*float);
    let negative = value.round() < 0.0;

    let float_round = value.round().copysign(1.0);

    let float_fract = value.fract().copysign(1.0);
    let exp = leading_zeros_exp(value);

    (
        negative,
        float_round as i32,
        float_fract as i32,
        exp as i32,
        leading_zeros_fractional(float),
    )
}
pub fn leading_zeros_fractional<T: Copy + Deref<Target = f64>>(float: T) -> usize {
    let s = float.to_string();
    if let Some(dot_idx) = s.find('.') {
        s[dot_idx + 1..].chars().take_while(|&c| c == '0').count()
    } else {
        0
    }
}
pub fn leading_zeros_exp<T: Copy + Deref<Target = f32>>(float: T) -> i32 {
    let float = *float;
    let float_fract_leading_zeroes = leading_zeros_fractional(&(float as f64));
    let float_fract = float.fract().copysign(1.0_f32);
    let mut exp = float.fract();
    for _ in 0..float_fract_leading_zeroes {
        exp = exp * 10.0;
    }
    assert!(exp >= 0.0, "expected {exp} to be >= 0.0");
    return exp as i32;
}

// pub fn leading_zeros_fractional(value: f64) -> (i32, i32) {
//     let s = value.to_string();
//     if let Some(dot_idx) = s.find('.') {
//         let chars = s[dot_idx + 1..].chars().collect::<Vec<char>>();
//         let zero_start = chars.iter().take_while(|&c| *c == '0').collect::<String>();
//         let right_value = chars.iter().skip_while(|&c| *c == '0').collect::<String>();
//         let zero_count = zero_start.len();
//         let nonzero_value = right_value.parse::<i32>().unwrap();
//         (zero_count as i32, nonzero_value)
//     } else {
//         (0, 0)
//     }
// }
