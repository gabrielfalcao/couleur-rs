use crate::{
    Contrast, ConversionToU8Error, Error, HEX_RGB_REGEX, Layer, RESET, Reset, Result, RgbTriple,
    Value, Wrap, max_rgb, min_rgb,
};
use regex::Regex;
use std::{ops::Deref, str::FromStr, sync::LazyLock};
use thiserror::Error as ThisError;

pub static BLACK: LazyLock<Color> =
    LazyLock::new(|| Color::new(0.0_f32, 0.0_f32, 0.0_f32).unwrap());
pub static WHITE: LazyLock<Color> =
    LazyLock::new(|| Color::new(255.0_f32, 255.0_f32, 255.0_f32).unwrap());
use terminal_colorsaurus::{QueryOptions, background_color, foreground_color};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct Color(pub Value, pub Value, pub Value);
impl Color {
    pub fn new<T: Copy + Into<f32>>(red: T, green: T, blue: T) -> Result<Color> {
        Ok(Color(
            Value::new(red.into())?,
            Value::new(green.into())?,
            Value::new(blue.into())?,
        ))
    }
    pub fn default_for_bg() -> Result<Color> {
        let terminal_bg_color = background_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }
    pub fn default_for_fg() -> Result<Color> {
        let terminal_bg_color = foreground_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }
    pub fn default_for_layer(layer: Layer) -> Result<Color> {
        Ok(match layer {
            Layer::BG => Self::default_for_bg()?,
            Layer::FG => Self::default_for_fg()?,
        })
    }
    pub fn red_value(&self) -> Value {
        self.0
    }
    pub fn green_value(&self) -> Value {
        self.1
    }
    pub fn blue_value(&self) -> Value {
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

    pub fn to_triple(&self) -> [Value; 3] {
        [self.red_value(), self.green_value(), self.blue_value()]
    }
    pub fn to_hex_string(&self) -> String {
        let [red, green, blue] = self.to_triple();
        format!("#{red:X}{green:X}{blue:X}")
    }
    pub fn from_triple(red: Value, green: Value, blue: Value) -> Color {
        Color(red, green, blue)
    }
    pub fn get_binary_contrast(&self) -> Color {
        let [r, g, b] = self.to_triple();
        let luminance = (0.299 * *r) + (0.587 * *g) + (0.114 * *b);
        if luminance > 128.0 { *BLACK } else { *WHITE }
    }

    pub fn get_msb_invert_contrast(&self) -> Color {
        Color(
            (self.red_value().into_u8() ^ 128).into(),
            (self.green_value().into_u8() ^ 128).into(),
            (self.blue_value().into_u8() ^ 128).into(),
        )
    }

    pub fn get_adobe_complementary(&self) -> Color {
        let [r, g, b] = self.to_triple();
        let max_val = max_rgb(r, g, b);
        let min_val = min_rgb(r, g, b);
        let target = max_val + min_val;
        Color(
            (target - r).copysign(&1.0).into(),
            (target - g).copysign(&1.0).into(),
            (target - b).copysign(&1.0).into(),
        )
    }

    pub fn get_wcag_luminance(&self) -> f32 {
        let [r, g, b] = self.to_triple();
        let channels = [(r / 255.0), (g / 255.0), (b / 255.0)];
        let mut linear = Vec::<Value>::new();

        for c in channels {
            if c <= 0.04045 {
                linear.push(Value(*(c / 12.92)))
            } else {
                linear.push(Value(*((c + 0.055) / 1.055) * 2.4))
            }
        }
        let luminance = 0.2126 * *linear[0] + 0.7152 * *linear[1] + 0.0722 * *linear[2];
        luminance
    }

    pub fn get_accessible_contrast(&self) -> Color {
        if self.get_wcag_luminance() > 0.175 {
            *BLACK
        } else {
            *WHITE
        }
    }
    pub fn to_ansi(&self, layer: Layer, bold: bool) -> String {
        let triple = self
            .to_triple()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
        let prefix = triple.join(";");
        let mut parts = if bold {
            vec!["1".to_string()]
        } else {
            Vec::<String>::new()
        };
        parts.push(layer.code().to_string());
        parts.push("2".to_string());
        parts.push(format!("{prefix}m"));
        format!("\x1b[{}", parts.join(";"))
    }

    pub fn wrap_ansi(
        &self,
        text: &str,
        layer: Option<Layer>,
        bold: bool,
        wrap: Option<Wrap>,
        reset: Option<Reset>,
        contrast: Option<Contrast>,
    ) -> String {
        let layer = layer.unwrap_or_default();
        let wrap = wrap.unwrap_or_default();
        let reset = reset.unwrap_or_default();
        let contrast = contrast.unwrap_or_default();

        let ansi_sequence = self.to_ansi(layer, bold);
        let contrast = if contrast != Contrast::None {
            self.contrast(contrast).to_ansi(layer.inverted(), bold)
        } else {
            String::new()
        };

        let colored = match wrap {
            Wrap::Before => format!("{ansi_sequence}{text}"),
            Wrap::After => format!("{text}{ansi_sequence}"),
            Wrap::Around => format!("{ansi_sequence}{text}{ansi_sequence}"),
        };
        let result = match reset {
            Reset::Before => format!("{RESET}{colored}"),
            Reset::After => format!("{colored}{RESET}"),
            Reset::Around => format!("{RESET}{colored}{RESET}"),
            Reset::None => colored,
        };
        return result;
    }
    pub fn contrast(&self, contrast: Contrast) -> Color {
        match contrast {
            Contrast::Read => self.get_accessible_contrast(),
            Contrast::HighBit => self.get_binary_contrast(),
            Contrast::Harmonic => self.get_adobe_complementary(),
            Contrast::Web => self.get_msb_invert_contrast(),
            Contrast::None => *self,
        }
    }
}
impl From<RgbTriple> for Color {
    fn from(triple: RgbTriple) -> Color {
        Color(
            Value::from(triple.red()),
            Value::from(triple.green()),
            Value::from(triple.blue()),
        )
    }
}
#[derive(Clone, Debug, ThisError)]
pub enum RGBParseError {
    #[error("failed to parse color {0}")]
    HexParseError(String),
}

impl<T> From<(T, T, T)> for Color
where
    T: Deref<Target = u8> + Copy,
{
    fn from(triple: (T, T, T)) -> Color {
        let (into_red, into_green, into_blue) = triple;
        let red = *into_red;
        let green = *into_green;
        let blue = *into_blue;
        Color(
            Value::from_u8(red).expect("red from u8"),
            Value::from_u8(green).expect("green from u8"),
            Value::from_u8(blue).expect("blue from u8"),
        )
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Color> {
        match HEX_RGB_REGEX.captures(s) {
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
                let r = Value::from_u8(red)?;
                let g = Value::from_u8(green)?;
                let b = Value::from_u8(blue)?;
                Ok(Color(r, g, b))
            }
            None => Err(RGBParseError::HexParseError(s.to_string()).into()),
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#{}", self.to_triple().iter().map(|c|format!("{:02X}", c.into_u8())).collect::<Vec<String>>().join(""))
    }
}

#[cfg(test)]
mod test {
    use crate::{Result, Error, Color};
    #[test]
    fn test_to_string() -> Result<()>{
        let color = "A4F681".parse::<Color>()?;
        assert_eq!(color.to_string(), "#A4F681");
        Ok(())
    }
}
