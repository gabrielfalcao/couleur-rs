use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    str::FromStr,
    sync::LazyLock,
};

use regex::Regex;
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de::{self, Error as SerdeError, Visitor},
};
use terminal_colorsaurus::{QueryOptions, background_color, foreground_color};
use thiserror::Error as ThisError;

use super::{BLACK, WHITE};
use crate::{
    AnsiRenderable,
    Contrast,
    ConversionToU8Error,
    Error,
    HEX_RGB_REGEX,
    Layer,
    Prefix,
    Reset,
    Result,
    Terminal,
    Value,
    Wrap,
    max_rgb,
    min_rgb,
};

/// Represents an RGB color, providing methods to obtain color
/// information and render the color in ANSI terminals supporting true-color.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Color(
    /// The **red** band
    pub Value,
    /// The **green** band
    pub Value,
    /// The **blue** band
    pub Value,
);
impl Color {
    /// Creates a new color from the given red, green and blue values which should be of a type convertible [`Into<f32>`]
    pub fn new<T: Copy + Into<f32>>(red: T, green: T, blue: T) -> Result<Color> {
        Ok(Color(Value::new(red.into())?, Value::new(green.into())?, Value::new(blue.into())?))
    }

    /// queries the [`Terminal`] for the background color,
    /// defaulting to [`BLACK`] in case
    /// [`Terminal::background_color`] fails.
    ///
    /// In other words, in case of querying the background color of
    /// the terminal fails, this method assumes the background is black.
    ///
    /// Because this method ignores terminal query errors you miss on
    /// the opportunity to handle errors and notify the user of your
    /// rust application or library, if this is the case you use the
    /// [`Terminal`] methods directly, in this case simply call
    /// [`Terminal::background_color()`].
    ///
    /// [`Terminal`]: crate::Terminal
    /// [`BLACK`]: crate::color::BLACK
    /// [`Terminal::background_color()`]: crate::Terminal::background_color
    /// [`Terminal::foreground_color()`]: crate::Terminal::foreground_color

    pub fn default_for_bg() -> Color {
        Terminal::background_color().unwrap_or_else(|_| BLACK)
    }

    /// queries the [`Terminal`] for the foreground color,
    /// defaulting to [`WHITE`] in case
    /// [`Terminal::foreground_color`] fails.
    ///
    /// In other words, in case of querying the foreground color of
    /// the terminal fails, this method assumes the foreground is black.
    ///
    /// Because this method ignores terminal query errors you miss on
    /// the opportunity to handle errors and notify the user of your
    /// rust application or library, if this is the case you use the
    /// [`Terminal`] methods directly, in this case simply call
    /// [`Terminal::foreground_color()`].
    ///
    /// [`Terminal`]: crate::Terminal
    /// [`WHITE`]: crate::color::WHITE
    /// [`Terminal::background_color()`]: crate::Terminal::background_color
    /// [`Terminal::foreground_color()`]: crate::Terminal::foreground_color
    pub fn default_for_fg() -> Color {
        Terminal::foreground_color().unwrap_or_else(|_| WHITE)
    }

    /// queries the [`Terminal`] for the color of the given
    /// [`Layer`], defaulting the background to black and the
    /// foreground to white in case of errors, which is the same
    /// behavior of the [`Terminal::background_color()`] and [`Terminal::foreground_color()`]
    /// methods.
    ///
    /// [`Terminal`]: crate::Terminal
    /// [`Layer`]: crate::Layer
    /// [`Terminal::background_color()`]: crate::Terminal::background_color
    /// [`Terminal::foreground_color()`]: crate::Terminal::foreground_color
    pub fn default_for_layer(layer: Layer) -> Color {
        Terminal::layer_color(layer).unwrap_or_else(|_| match layer {
            Layer::BG => BLACK,
            Layer::FG => WHITE,
        })
    }

    /// Returns the raw [`Value`] for the red band of this [`Color`]
    ///
    /// [`Color`]: crate::Color
    /// [`Value`]: crate::Value
    pub fn red_value(&self) -> Value {
        self.0
    }

    /// Returns the raw [`Value`] for the green band of this [`Color`]
    ///
    /// [`Color`]: crate::Color
    /// [`Value`]: crate::Value
    pub fn green_value(&self) -> Value {
        self.1
    }

    /// Returns the raw [`Value`] for the blue band of this [`Color`]
    ///
    /// [`Color`]: crate::Color
    /// [`Value`]: crate::Value
    pub fn blue_value(&self) -> Value {
        self.2
    }

    /// Returns the raw [`f32`] for the red band of this [`Color`]
    ///
    /// [`Color`]: crate::Color
    /// [`Value`]: crate::Value
    pub fn red(&self) -> f32 {
        self.red_value().value()
    }

    /// Returns the raw [`f32`] for the green band of this [`Color`]
    ///
    /// [`Color`]: crate::Color
    /// [`Value`]: crate::Value
    pub fn green(&self) -> f32 {
        self.green_value().value()
    }

    /// Returns the raw [`f32`] for the blue band of this [`Color`]
    ///
    /// [`Color`]: crate::Color
    /// [`Value`]: crate::Value
    pub fn blue(&self) -> f32 {
        self.blue_value().value()
    }

    /// Returns the a sized array of values of the 3 bands - red, green blue - for this color.
    pub fn to_triple(&self) -> [Value; 3] {
        [self.red_value(), self.green_value(), self.blue_value()]
    }

    /// Returns a string with the hex RGB representation of this color
    pub fn to_hex_string(&self) -> String {
        let [red, green, blue] = self.to_triple();
        format!("#{red:X}{green:X}{blue:X}")
    }

    /// Returns a [`Color`] from three [`Value`] values representing red, green and blue.
    pub fn from_triple(red: Value, green: Value, blue: Value) -> Color {
        Color(red, green, blue)
    }

    /// Returns the luminance of the color based on the binary
    /// luminance algorithm which reduces the color to either black or
    /// white based on human perception of color brightness.
    pub fn get_binary_luminance(&self) -> Value {
        let [r, g, b] = self.to_triple();
        let luminance = (0.299 * *r) + (0.587 * *g) + (0.114 * *b);
        Value::from_f32(luminance).expect("value between 0 and 255 inclusive")
    }

    /// Returns either [`BLACK`] or [`WHITE`] as the contrasting color based on the [`binary luminance algorithm`](`crate::Color::get_binary_luminance`)
    pub fn get_binary_contrast(&self) -> Color {
        if self.is_dark() { BLACK } else { WHITE }
    }

    /// Returns the contrast of the current color based on the simple
    /// application of the MSB (Most significant bit) algorithm. This
    /// method is probably the less sophisticated among the other
    /// color contrast algorithms methods of [`Color`] but is here as
    /// an option since it is so basic and common.
    pub fn get_msb_invert_contrast(&self) -> Color {
        Color(
            (self.red_value().into_u8() ^ 128).into(),
            (self.green_value().into_u8() ^ 128).into(),
            (self.blue_value().into_u8() ^ 128).into(),
        )
    }

    /// Returns a complementary color according to one of Adobe's
    /// complementary color algorithm.
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

    /// Returns the perceived brightness via WCAG (Web Content
    /// Accessibility Guidelines) luminance.
    ///
    /// A value above 0.175 is considered bright and a value below that
    /// is considered dark.
    ///
    /// This math is the basis for determining whether text and its
    /// background have enough contrast to be readable by users with
    /// low vision or color blindness
    pub fn get_wcag_luminance(&self) -> Value {
        let [r, g, b] = self.to_triple();
        let channels = [(r / 255.0), (g / 255.0), (b / 255.0)];
        let mut linear = Vec::<Value>::new();

        for c in channels {
            if c <= 0.04045 {
                linear
                    .push(Value::from_f32(*(c / 12.92)).expect("value between 0 and 255 inclusive"))
            } else {
                linear.push(
                    Value::from_f32(*((c + 0.055) / 1.055) * 2.4)
                        .expect("value between 0 and 255 inclusive"),
                )
            }
        }
        let luminance = 0.2126 * *linear[0] + 0.7152 * *linear[1] + 0.0722 * *linear[2];
        Value::from_f32(luminance).expect("value between 0 and 255 inclusive")
    }

    /// Returns either [`BLACK`] or [`WHITE`] as the contrast of the
    /// current color.
    pub fn get_accessible_contrast(&self) -> Color {
        if self.get_wcag_luminance() > 0.175 { BLACK } else { WHITE }
    }

    /// Returns a string which renders the current color as an ANSI sequence in the given [`Layer`].
    pub fn to_ansi(&self, layer: Layer) -> String {
        self.to_ansi_with_prefix(layer, None)
    }

    /// Returns a string which renders the current color as an ANSI sequence in the given [`Layer`] and [`Prefix`].
    pub fn to_ansi_with_prefix(&self, layer: Layer, prefix: Option<Prefix>) -> String {
        let triple = self.to_triple().iter().map(|v| v.to_string()).collect::<Vec<String>>();
        let color = triple.join(";");
        let mut parts = Vec::<String>::new();
        parts.push(layer.code().to_string());
        parts.push("2".to_string());
        parts.push(format!("{color}m"));
        format!("{prefix}[{code}", prefix = prefix.unwrap_or_default(), code = parts.join(";"))
    }

    pub fn wrap_ansi(
        &self,
        text: &str,
        prefix: Option<Prefix>,
        layer: Option<Layer>,
        wrap: Option<Wrap>,
        reset: Option<Reset>,
        contrast: Option<Contrast>,
    ) -> String {
        let layer = layer.unwrap_or_default();
        let wrap = wrap.unwrap_or_default();
        let reset = reset.unwrap_or_default();
        let contrast = contrast.unwrap_or_default();

        let ansi_sequence = self.to_ansi_with_prefix(layer, prefix);
        let contrast = if contrast != Contrast::None {
            self.contrast(contrast).to_ansi_with_prefix(layer.inverted(), prefix)
        } else {
            String::new()
        };

        let colored = match wrap {
            Wrap::Before => format!("{ansi_sequence}{text}"),
            Wrap::After => format!("{text}{ansi_sequence}"),
            Wrap::Around => format!("{ansi_sequence}{text}{ansi_sequence}"),
        };
        let result = match reset {
            Reset::Before => format!("{reset}{colored}", reset = Reset::code()),
            Reset::After => format!("{colored}{reset}", reset = Reset::code()),
            Reset::Around => format!("{reset}{colored}{reset}", reset = Reset::code()),
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

    /// uses the [`binary luminance`](`Color::get_binary_luminance`)
    /// algorithm to determine if a [`Color`] is dark
    pub fn is_dark(&self) -> bool {
        self.get_binary_luminance() <= 128.0
    }

    /// uses the [`binary luminance`](`Color::get_binary_luminance`)
    /// algorithm to determine if a [`Color`] is light
    pub fn is_light(&self) -> bool {
        self.get_binary_luminance() > 128.0
    }

    /// uses the [`binary luminance`](`Color::get_binary_luminance`)
    /// algorithm to determine the given [`Color`] contrasts with the
    /// current color.
    pub fn contrasts_with_color(&self, other: Color) -> bool {
        if self.is_dark() {
            return other.is_light();
        } else {
            return other.is_dark();
        }
    }

    /// uses the [`binary luminance`](`Color::get_binary_luminance`)
    /// algorithm to determine the current color contrasts with the
    /// [`terminal background`](`Terminal::background_color`).
    ///
    /// Note that the background color used for comparison might be
    /// [`BLACK`] if querying the terminal fails. In other words, this
    /// method does not use [`Terminal::background_color`] directly
    /// but instead uses [`Color::default_for_bg`].
    ///
    /// Check the documentation of [`default_for_bg`] for more
    /// information.
    ///
    /// [`Terminal`]: crate::Terminal
    /// [`BLACK`]: crate::color::BLACK
    /// [`WHITE`]: crate::color::WHITE
    /// [`Terminal::background_color()`]: crate::Terminal::background_color
    /// [`Terminal::foreground_color()`]: crate::Terminal::foreground_color
    /// [`default_for_bg`]: crate::Color::default_for_bg
    /// [`default_for_fg`]: crate::Color::default_for_fg
    pub fn contrasts_with_background(&self) -> bool {
        let terminal_background = Color::default_for_bg();
        self.contrasts_with_color(terminal_background)
    }

    /// uses the [`binary luminance`](`Color::get_binary_luminance`)
    /// algorithm to determine the current color contrasts with the
    /// [`terminal foreground`](`Terminal::foreground_color`)
    ///
    /// Note that the foreground color used for comparison might be
    /// [`BLACK`] if querying the terminal fails. In other words, this
    /// method does not use [`Terminal::foreground_color`] directly
    /// but instead uses [`Color::default_for_fg`].
    ///
    /// Check the documentation of [`default_for_fg`] for more
    /// information.
    ///
    /// [`Terminal`]: crate::Terminal
    /// [`BLACK`]: crate::color::BLACK
    /// [`WHITE`]: crate::color::WHITE
    /// [`Terminal::background_color()`]: crate::Terminal::background_color
    /// [`Terminal::foreground_color()`]: crate::Terminal::foreground_color
    /// [`default_for_bg`]: crate::Color::default_for_bg
    /// [`default_for_fg`]: crate::Color::default_for_fg
    pub fn contrasts_with_foreground(&self) -> bool {
        let terminal_foreground = Color::default_for_fg();
        self.contrasts_with_color(terminal_foreground)
    }

    pub fn to_rgb_hex(&self) -> String {
        format!(
            "#{}",
            self.to_triple()
                .iter()
                .map(|c| format!("{:02X}", c.into_u8()))
                .collect::<Vec<String>>()
                .join("")
        )
    }
}
impl From<RgbTriple> for Color {
    fn from(triple: RgbTriple) -> Color {
        Color(Value::from(triple.red()), Value::from(triple.green()), Value::from(triple.blue()))
    }
}

/// Represents a failure to parse a [`Color`] from strings.
///
/// This enum is the `Err` error type used in the [`std::str::FromStr#required-associated-types`] implementation for [`Color`]
#[derive(Clone, Debug, ThisError, Serialize, Deserialize)]
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
                let red_value = captures.name("red").map(|s| s.as_str().to_string()).expect("red");
                let green_value =
                    captures.name("green").map(|s| s.as_str().to_string()).expect("green");
                let blue_value =
                    captures.name("blue").map(|s| s.as_str().to_string()).expect("blue");
                let red = u8::from_str_radix(
                    &captures.name("red").map(|s| s.as_str().to_string()).unwrap(),
                    16,
                )?;
                let green = u8::from_str_radix(
                    &captures.name("green").map(|s| s.as_str().to_string()).unwrap(),
                    16,
                )?;
                let blue = u8::from_str_radix(
                    &captures.name("blue").map(|s| s.as_str().to_string()).unwrap(),
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
        write!(f, "{}", self.to_rgb_hex())
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_triple().hash(state);
        self.to_rgb_hex().hash(state);
    }
}
impl AnsiRenderable for Color {
    fn render_without_prefix(&self) -> String {
        let triple = self.to_triple().iter().map(|v| v.to_string()).collect::<Vec<String>>();
        let color = triple.join(";");
        format!("2;{color}m")
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//  <<<  SSSSS   EEEEEEE  RRRRRR   DDDDD    EEEEEEE >>>
// <<<  SS       EE       RR   RR  DD  DD   EE       >>>
// <<<   SSSSS   EEEEE    RRRRRR   DD   DD  EEEEE    >>>
// <<<       SS  EE       RR  RR   DD   DD  EE       >>>
//  <<<  SSSSS   EEEEEEE  RR   RR  DDDDDD   EEEEEEE >>>

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.to_hex_string();

        serializer.serialize_str(&value)
    }
}

struct ValueVisitor;

impl<'de> Visitor<'de> for ValueVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a hexadecimal number of length 2")
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
    where
        E: SerdeError,
    {
        match Color::from_str(value) {
            Ok(value) => Ok(value),
            Err(error) => Err(E::custom(format!("{error}"))),
        }
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Self::Value, E>
    where
        E: SerdeError,
    {
        self.visit_str(&value)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ValueVisitor)
    }
}
//  <<<     //  SSSSS   EEEEEEE  RRRRRR   DDDDD    EEEEEEE >>>
// <<<     /// SS       EE       RR   RR  DD  DD   EE       >>>
// <<<    ///   SSSSS   EEEEE    RRRRRR   DD   DD  EEEEE    >>>
// <<<   ///        SS  EE       RR  RR   DD   DD  EE       >>>
//  <<< ///     SSSSS   EEEEEEE  RR   RR  DDDDDD   EEEEEEE >>>
//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// U8Triple is a type alias which represents a tuple containing the three RGB bands as [`u8`]
pub type U8Triple = (u8, u8, u8);

/// RgbTriple is an intermediary container which holds each of the
/// three bands of an RGB color as [`u8`] values.
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct RgbTriple(u8, u8, u8);

impl RgbTriple {
    pub fn red(&self) -> u8 {
        self.0
    }

    pub fn green(&self) -> u8 {
        self.1
    }

    pub fn blue(&self) -> u8 {
        self.2
    }

    pub fn into_triple(self) -> U8Triple {
        (self.red(), self.green(), self.blue())
    }
}
impl From<U8Triple> for RgbTriple {
    fn from(input: U8Triple) -> RgbTriple {
        let (red, green, blue) = input;
        RgbTriple(red, green, blue)
    }
}

impl Into<U8Triple> for RgbTriple {
    fn into(self) -> U8Triple {
        self.into_triple()
    }
}

#[cfg(test)]
mod test {
    use crate::{Color, Error, Result};
    #[test]
    fn test_to_string() -> Result<()> {
        let color = "A4F681".parse::<Color>()?;
        assert_eq!(color.to_string(), "#A4F681");
        Ok(())
    }
    #[test]
    fn test_get_binary_luminance() -> Result<()> {
        let color_547e64 = "#547E64".parse::<Color>()?;
        assert_eq!(color_547e64.is_dark(), true);
        assert_eq!(color_547e64.is_light(), false);

        let color_374e4a = "#374E4A".parse::<Color>()?;
        assert_eq!(color_374e4a.is_dark(), true);
        assert_eq!(color_374e4a.is_light(), false);

        let color_92a984 = "#92A984".parse::<Color>()?;
        assert_eq!(color_92a984.is_dark(), false);
        assert_eq!(color_92a984.is_light(), true);

        let color_b2ba90 = "#B2BA90".parse::<Color>()?;
        assert_eq!(color_b2ba90.is_dark(), false);
        assert_eq!(color_b2ba90.is_light(), true);

        let color_cddf6c = "#CDDF6C".parse::<Color>()?;
        assert_eq!(color_cddf6c.is_dark(), false);
        assert_eq!(color_cddf6c.is_light(), true);

        Ok(())
    }
}
