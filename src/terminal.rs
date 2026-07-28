use terminal_colorsaurus::{QueryOptions, background_color, foreground_color};

use crate::{Color, Error, Layer, Result, Value};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct Terminal;
impl Terminal {
    pub fn background_color() -> Result<Color> {
        let terminal_bg_color = background_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }

    pub fn foreground_color() -> Result<Color> {
        let terminal_bg_color = foreground_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }

    pub fn layer_color(layer: Layer) -> Result<Color> {
        Ok(match layer {
            Layer::BG => Self::background_color()?,
            Layer::FG => Self::foreground_color()?,
        })
    }

    pub fn background_luminance() -> Result<Value> {
        let background_color = Self::background_color()?;
        Ok(background_color.get_binary_luminance().into())
    }

    pub fn foreground_luminance() -> Result<Value> {
        let foreground_color = Self::foreground_color()?;
        Ok(foreground_color.get_binary_luminance().into())
    }

    pub fn is_dark() -> Result<bool> {
        let luminance = Self::background_luminance()?;
        Ok(luminance < 128.0)
    }

    pub fn is_light() -> Result<bool> {
        let luminance = Self::background_luminance()?;
        Ok(luminance >= 128.0)
    }
    pub fn info() -> TerminalInfoResult {
        let background = match Terminal::background_color() {
            Ok(color) => color,
            Err(error) => return TerminalInfoResult::Error(Layer::BG, error.to_string().leak()),
        };
        let foreground = match Terminal::foreground_color() {
            Ok(color) => color,
            Err(error) => return TerminalInfoResult::Error(Layer::FG, error.to_string().leak()),
        };

        let is_dark = background.is_dark();
        let is_light = background.is_light();
        let binary_luminance = background.get_binary_luminance();
        let wcag_luminance = background.get_wcag_luminance();
        let info = TerminalInfo {
            background,
            foreground,
            is_dark,
            is_light,
            binary_luminance,
            wcag_luminance,
        };
        TerminalInfoResult::Info(info)
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub enum TerminalInfoResult {
    Info(TerminalInfo),
    Error(Layer, &'static str),
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord)]
pub struct TerminalInfo {
    pub background: Color,
    pub foreground: Color,
    pub is_dark: bool,
    pub is_light: bool,
    pub binary_luminance: Value,
    pub wcag_luminance: Value,
}
