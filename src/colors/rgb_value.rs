use crate::float::{leading_zeros_exp, leading_zeros_fractional};
use crate::{
    ConversionToU8Error, Error, FloatMetadata, Result, SINGLE_BAND_DECIMAL_RGB_REGEX,
    SINGLE_BAND_HEX_RGB_REGEX, impl_op,
};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Div,
    DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
pub struct RGBValue(pub f32);

impl RGBValue {
    pub fn new<T: Copy + Into<f32>>(value: T) -> RGBValue {
        RGBValue::from_f32(value.into())
    }
    pub fn from_f32(value: f32) -> RGBValue {
        RGBValue::new(value)
    }
    pub fn value(&self) -> f32 {
        self.0
    }
    pub fn round(&self) -> f32 {
        self.value().round()
    }
    pub fn fract(&self) -> f32 {
        self.value().fract()
    }
    pub fn copysign<T: Copy + Deref<Target = f32>>(&self, other: T) -> f32 {
        self.value().copysign(*other)
    }
    pub fn to_u8(&self) -> Result<u8> {
        let value = self.value().clamp(0.0, 255.0);
        if value > 255.0 {
            Err(Error::ConversionToU8Error(ConversionToU8Error(
                value,
                format!(
                    "cannot convert {value} to u8 because the value is out of boundary: {value} > 255",
                    value = self.0
                ),
            ).to_string()))
        } else if value < 0.0 {
            Err(Error::ConversionToU8Error(ConversionToU8Error(
                value,
                format!(
                    "cannot convert {value} to u8 because the value is out of boundary: {value} < 0",
                    value = self.0
                ),
            ).to_string()))
        } else {
            Ok(value.ceil() as u8)
        }
    }
    pub fn into_u8(self) -> u8 {
        self.to_u8().expect(&format!(
            "RGB value to be within 0 and 255 but is {value}",
            value = self.0
        ))
    }
    pub fn from_u8(value: u8) -> RGBValue {
        RGBValue(value as f32)
    }
    fn leading_zeros_fractional(&self) -> usize {
        let s = self.to_string();
        if let Some(dot_idx) = s.find('.') {
            s[dot_idx + 1..].chars().take_while(|&c| c == '0').count()
        } else {
            0
        }
    }
    pub fn leading_zeros_exp(&self) -> (i32) {
        let self_fract_leading_zeroes = self.leading_zeros_fractional();
        let self_fract = self.fract().copysign(1.0_f32);
        let mut exp = self.fract();
        for _ in 0..self_fract_leading_zeroes {
            exp = exp * 10.0;
        }
        assert!(exp >= 0.0, "expected {exp} to be >= 0.0");
        return exp as i32;
    }

    pub fn float_metadata(&self) -> (bool, i32, i32, i32, usize) {
        let negative = self.round() < 0.0;

        let float_round = self.round().copysign(1.0);

        let float_fract = self.fract().copysign(1.0);
        let exp = self.leading_zeros_exp();

        (
            negative,
            float_round as i32,
            float_fract as i32,
            exp as i32,
            self.leading_zeros_fractional(),
        )
    }
    pub fn to_hex_rgb(&self) -> String {
        let red = self.value() as u8;
        format!("#{red:02X}B08B")
    }
}

impl Deref for RGBValue {
    type Target = f32;

    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl FromStr for RGBValue {
    type Err = Error;
    fn from_str(value: &str) -> std::result::Result<RGBValue, Error> {
        match SINGLE_BAND_HEX_RGB_REGEX.captures(value) {
            Some(caps) => {
                let band = caps.name("band").expect("rgb band").as_str();
                let parsed = u8::from_str_radix(band, 16)?;
                Ok(RGBValue(parsed as f32))
            }
            None => match SINGLE_BAND_DECIMAL_RGB_REGEX.captures(value) {
                Some(band) => match band.name("band") {
                    Some(band) => {
                        let parsed = u8::from_str_radix(band.as_str(), 16)?;
                        Ok(RGBValue(parsed as f32))
                    }
                    None => Err(Error::ParseError(format!(
                        "cannot parse RGB value (number from 0 to 255) from {value:#?}"
                    ))),
                },
                None => Err(Error::ParseError(format!(
                    "cannot parse RGB value (number from 0 to 255) from {value:#?}"
                ))),
            },
        }
    }
}

impl std::fmt::Display for RGBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = self.value();
        write!(f, "{value}")
    }
}
impl std::fmt::LowerHex for RGBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = self.into_u8();
        write!(f, "{value:02x}")
    }
}
impl std::fmt::UpperHex for RGBValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value = self.into_u8();
        write!(f, "{value:02X}")
    }
}

impl Into<f32> for RGBValue {
    fn into(self) -> f32 {
        self.value()
    }
}

impl Into<u8> for RGBValue {
    fn into(self) -> u8 {
        self.into_u8()
    }
}
impl Into<u16> for RGBValue {
    fn into(self) -> u16 {
        u16::from(self.into_u8())
    }
}
impl Into<u32> for RGBValue {
    fn into(self) -> u32 {
        u32::from(self.into_u8())
    }
}
impl Into<u64> for RGBValue {
    fn into(self) -> u64 {
        u64::from(self.into_u8())
    }
}
impl Into<usize> for RGBValue {
    fn into(self) -> usize {
        usize::from(self.into_u8())
    }
}

impl From<f32> for RGBValue {
    fn from(value: f32) -> RGBValue {
        RGBValue(value)
    }
}
impl From<i32> for RGBValue {
    fn from(val: i32) -> RGBValue {
        let value = (val % 255i32) as f32;
        RGBValue(value)
    }
}
impl From<i64> for RGBValue {
    fn from(val: i64) -> RGBValue {
        let value = (val % 255i64) as f32;
        RGBValue(value)
    }
}
impl From<u8> for RGBValue {
    fn from(val: u8) -> RGBValue {
        RGBValue(val as f32)
    }
}
impl From<u16> for RGBValue {
    fn from(val: u16) -> RGBValue {
        RGBValue(val as f32)
    }
}

impl From<u32> for RGBValue {
    fn from(val: u32) -> RGBValue {
        RGBValue(val as f32)
    }
}
impl From<u64> for RGBValue {
    fn from(val: u64) -> RGBValue {
        RGBValue(val as f32)
    }
}
impl From<usize> for RGBValue {
    fn from(val: usize) -> RGBValue {
        RGBValue(val as f32)
    }
}

impl<T> PartialEq<T> for RGBValue
where
    T: Into<RGBValue> + Copy,
{
    fn eq(&self, other: &T) -> bool {
        let rhs = Into::<RGBValue>::into(*other);
        self.float_metadata().eq(&rhs.float_metadata())
    }
}
impl Eq for RGBValue {}

impl<T> PartialOrd<T> for RGBValue
where
    T: Into<RGBValue> + Copy,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let rhs = Into::<RGBValue>::into(*other);
        self.float_metadata().partial_cmp(&rhs.float_metadata())
    }
}
impl Ord for RGBValue {
    fn cmp(&self, other: &RGBValue) -> Ordering {
        self.float_metadata().cmp(&other.float_metadata())
    }
}

impl_op!(Add, add, value, +);
impl_op!(Sub, sub, value, -);
impl_op!(Div, div, value, /);
impl_op!(Mul, mul, value, *);
impl_op!(Rem, rem, value, %);
