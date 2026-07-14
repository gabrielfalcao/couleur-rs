use crate::{Algorithm, ConversionToU8Error, Error, RGBValue, Result, RgbTriple, max_rgb, min_rgb};
use owo_colors::Rgb;
use regex::Regex;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::LazyLock;
use thiserror::Error as ThisError;
pub static BLACK: LazyLock<RGBColor> =
    LazyLock::new(|| RGBColor::new(0.0_f32, 0.0_f32, 0.0_f32).unwrap());
pub static WHITE: LazyLock<RGBColor> =
    LazyLock::new(|| RGBColor::new(255.0_f32, 255.0_f32, 255.0_f32).unwrap());

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct RGBColor(pub RGBValue, pub RGBValue, pub RGBValue);
impl RGBColor {
    pub fn contrast<T: Algorithm>(&self, algo: T) -> RGBColor {
        return algo.apply(*self);
    }
}
impl RGBColor {
    pub fn new<T: Copy + Into<f32>>(red: T, green: T, blue: T) -> Result<RGBColor> {
        Ok(RGBColor(
            RGBValue::new(red.into())?,
            RGBValue::new(green.into())?,
            RGBValue::new(blue.into())?,
        ))
    }
    pub fn red_value(&self) -> RGBValue {
        self.0
    }
    pub fn green_value(&self) -> RGBValue {
        self.1
    }
    pub fn blue_value(&self) -> RGBValue {
        self.2
    }

    pub fn red(&self) -> f32 {
        self.red_value().value()
    }
    pub fn green(&self) -> f32 {
        self.green_value().value()
    }
    pub fn blue(&self) -> f32 {
        self.blue_value().value()
    }

    pub fn to_triple(&self) -> (RGBValue, RGBValue, RGBValue) {
        (self.red_value(), self.green_value(), self.blue_value())
    }
    pub fn to_hex_string(&self) -> String {
        let (red, green, blue) = self.to_triple();
        format!("#{red:X}{green:X}{blue:X}")
    }
    pub fn from_triple(red: RGBValue, green: RGBValue, blue: RGBValue) -> RGBColor {
        RGBColor(red, green, blue)
    }
    pub fn get_binary_contrast(&self) -> RGBColor {
        let (r, g, b) = self.to_triple();
        let luminance = (0.299 * *r) + (0.587 * *g) + (0.114 * *b);
        if luminance > 128.0 { *BLACK } else { *WHITE }
    }

    pub fn get_msb_invert_contrast(&self) -> RGBColor {
        RGBColor(
            (self.red_value().into_u8() ^ 128).into(),
            (self.green_value().into_u8() ^ 128).into(),
            (self.blue_value().into_u8() ^ 128).into(),
        )
    }

    pub fn get_adobe_complementary(&self) -> RGBColor {
        let (r, g, b) = self.to_triple();
        let max_val = max_rgb(r, g, b);
        let min_val = min_rgb(r, g, b);
        let target = max_val + min_val;
        RGBColor(
            (target - r).copysign(&1.0).into(),
            (target - g).copysign(&1.0).into(),
            (target - b).copysign(&1.0).into(),
        )
    }

    pub fn get_wcag_luminance(&self) -> f32 {
        let (r, g, b) = self.to_triple();
        let channels = [(r / 255.0), (g / 255.0), (b / 255.0)];
        let mut linear = Vec::<RGBValue>::new();

        for c in channels {
            if c <= 0.04045 {
                linear.push(RGBValue(*(c / 12.92)))
            } else {
                linear.push(RGBValue(*((c + 0.055) / 1.055) * 2.4))
            }
        }
        let luminance = 0.2126 * *linear[0] + 0.7152 * *linear[1] + 0.0722 * *linear[2];
        luminance
    }

    pub fn get_accessible_contrast(&self) -> RGBColor {
        if self.get_wcag_luminance() > 0.175 {
            *BLACK
        } else {
            *WHITE
        }
    }
}
impl From<RgbTriple> for RGBColor {
    fn from(triple: RgbTriple) -> RGBColor {
        RGBColor(
            RGBValue::from(triple.red()),
            RGBValue::from(triple.green()),
            RGBValue::from(triple.blue()),
        )
    }
}
#[derive(Clone, Debug, ThisError)]
pub enum RGBParseError {
    #[error("failed to parse color {0}")]
    HexParseError(String),
}

impl<T> From<(T, T, T)> for RGBColor
where
    T: Deref<Target = u8> + Copy,
{
    fn from(triple: (T, T, T)) -> RGBColor {
        let (into_red, into_green, into_blue) = triple;
        let red = *into_red;
        let green = *into_green;
        let blue = *into_blue;
        RGBColor(
            RGBValue::from_u8(red).expect("red from u8"),
            RGBValue::from_u8(green).expect("green from u8"),
            RGBValue::from_u8(blue).expect("blue from u8"),
        )
    }
}

pub static RGB_COLOR_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?<red>[a-fA-F0-9]{1,2})(?<green>[a-fA-F0-9]{1,2})(?<blue>[a-fA-F0-9]{1,2})")
        .unwrap()
});

impl FromStr for RGBColor {
    type Err = Error;

    fn from_str(s: &str) -> Result<RGBColor> {
        match RGB_COLOR_REGEX.captures(s) {
            Some(captures) => {
                let red_value = captures
                    .name("red")
                    .map(|s| s.as_str().to_string())
                    .expect("red");
                let green_value = captures
                    .name("green")
                    .map(|s| s.as_str().to_string())
                    .expect("green");
                let blue_value = captures
                    .name("blue")
                    .map(|s| s.as_str().to_string())
                    .expect("blue");
                let red = u8::from_str_radix(
                    &captures
                        .name("red")
                        .map(|s| s.as_str().to_string())
                        .unwrap(),
                    16,
                )?;
                let green = u8::from_str_radix(
                    &captures
                        .name("green")
                        .map(|s| s.as_str().to_string())
                        .unwrap(),
                    16,
                )?;
                let blue = u8::from_str_radix(
                    &captures
                        .name("blue")
                        .map(|s| s.as_str().to_string())
                        .unwrap(),
                    16,
                )?;
                let r = RGBValue::from_u8(red)?;
                let g = RGBValue::from_u8(green)?;
                let b = RGBValue::from_u8(blue)?;
                Ok(RGBColor(r, g, b))
            }
            None => Err(RGBParseError::HexParseError(s.to_string()).into()),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{RGBColor, RGBValue, Result};
    use k9::assert_equal;
    use std::cmp::{max, min};

    #[test]
    fn test_parse() -> Result<()> {
        let dark_pink = "#C32454".parse::<RGBColor>()?;
        let darkest_pink = "#831C5D".parse::<RGBColor>()?;
        let light_pink = "#FCA790".parse::<RGBColor>()?;
        let lightest_pink = "#FDCBB0".parse::<RGBColor>()?;

        assert_eq!(
            dark_pink.to_triple(),
            (
                RGBValue::from_u8(0xC3)?,
                RGBValue::from_u8(0x24)?,
                RGBValue::from_u8(0x54)?
            )
        );

        Ok(())
    }
    #[test]
    fn test_parse_and_get_accessible_contrast() -> Result<()> {
        // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
        // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
        // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
        // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
        // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
        let lightest: RGBColor = "#8FF8E2".parse()?;
        let darkest: RGBColor = "#0B5E65".parse()?;
        assert_equal!(
            lightest.get_accessible_contrast(),
            RGBColor::from_triple(0.into(), 0.into(), 0.into())
        );
        assert_equal!(
            darkest.get_accessible_contrast(),
            RGBColor::from_triple(255.into(), 255.into(), 255.into())
        );
        Ok(())
    }
    #[test]
    fn test_parse_and_get_binary_contrast() -> Result<()> {
        // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
        // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
        // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
        // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
        // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
        let lightest: RGBColor = "#8FF8E2".parse()?;
        let darkest: RGBColor = "#0B5E65".parse()?;
        assert_equal!(
            lightest.get_binary_contrast(),
            RGBColor::from_triple(0.into(), 0.into(), 0.into())
        );
        assert_equal!(
            darkest.get_binary_contrast(),
            RGBColor::from_triple(255.into(), 255.into(), 255.into())
        );
        Ok(())
    }

    #[test]
    fn test_parse_and_get_adobe_complementary() -> Result<()> {
        // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
        // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
        // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
        // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
        // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
        let lightest: RGBColor = "#8FF8E2".parse()?;
        let darkest: RGBColor = "#0B5E65".parse()?;
        assert_equal!(
            lightest.get_adobe_complementary(),
            RGBColor::from_triple(248.into(), 143.into(), 165.into())
        );
        assert_equal!(
            darkest.get_adobe_complementary(),
            RGBColor::from_triple(101.into(), 18.into(), 11.into())
        );
        Ok(())
    }
    #[test]
    fn test_parse_and_get_msb_invert_contrast() -> Result<()> {
        // #0B5E65  \x1b[1;38;2;11;94;101m     11,  94, 101
        // #0B8A8F  \x1b[1;38;2;11;138;143m    11, 138, 143
        // #0EAF9B  \x1b[1;38;2;14;175;155m    14, 175, 155
        // #30E1B9  \x1b[1;38;2;48;225;185m    48, 225, 185
        // #8FF8E2  \x1b[1;38;2;143;248;226m  143, 248, 226
        let lightest: RGBColor = "#8FF8E2".parse()?;
        let darkest: RGBColor = "#0B5E65".parse()?;
        assert_equal!(
            lightest.get_msb_invert_contrast(),
            RGBColor::from_triple(15.into(), 120.into(), 98.into())
        );
        assert_equal!(
            darkest.get_msb_invert_contrast(),
            RGBColor::from_triple(139.into(), 222.into(), 229.into())
        );
        Ok(())
    }
}
