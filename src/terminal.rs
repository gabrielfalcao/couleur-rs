use crate::{
    Color,
    Error,
    Layer,
    Result,
    Value,
};
use serde::{Deserialize, Serialize};
use terminal_colorsaurus::{QueryOptions, background_color, foreground_color};

/// Terminal is an abstraction to retrive information regarding the
/// background and foreground colors of the terminal at runtime along
/// with information such as whether the background is dark or light
/// and the binary luminance of both.
///
/// The [`Terminal::info()`] method returns a [`TerminalInfo`] struct with all the
/// information at only one place so that the terminal need not be
/// queried every time for the same information since the terminal's
/// background and foreground color tend not do change during runtime.
///
/// A [`TerminalInfo`] may be valid or invalid: an invalid
/// [`TerminalInfo`] contains the error message and the layer for
/// which querying failed, those two details are available in the
/// `Details` variant of [`TerminalInfoError`].  Conversely, when
/// querying the [`Terminal::info()`] succeeds, the resulting [`TerminalInfo`] is
/// **valid** and its `error` field is the `None` variant of
/// [`TerminalInfoError`].
///
/// Note that instead of using [`Terminal`] directly you can simply
/// use [`crate::TERMINAL`] which is a static variable with
/// information queried at the beginning of the runtime of your rust
/// application or library.
///
/// [`Terminal::info()`]: crate::Terminal::info
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, Default)]
pub struct Terminal;

impl Terminal {
    /// returns the background color of the [`Terminal`]
    pub fn background_color() -> Result<Color> {
        let terminal_bg_color = background_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }

    /// returns the foreground color of the [`Terminal`]
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

    /// returns a [`TerminalInfo`] by querying the terminal's
    /// background and foreground colors and computing their luminance
    /// to obtain all the details in [`TerminalInfo`].
    pub fn info() -> TerminalInfo {
        let background = match Terminal::background_color() {
            Ok(color) => color,
            Err(error) => return TerminalInfo::invalid(Layer::BG, error),
        };
        let foreground = match Terminal::foreground_color() {
            Ok(color) => color,
            Err(error) => return TerminalInfo::invalid(Layer::FG, error),
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
            is_valid: true,
            error: TerminalInfoError::None,
        };
        info
    }
}

/// Holds error details which may occur while querying terminal colors via [`Terminal::background_color`] or  [`Terminal::foreground_color`].
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub enum TerminalInfoError {
    None,
    Details { layer: Layer, message: String },
}
impl TerminalInfoError {
    pub fn is_none(&self) -> bool {
        match self {
            TerminalInfoError::None => true,
            TerminalInfoError::Details { .. } => false,
        }
    }
    pub fn has_details(&self) -> bool {
        match self {
            TerminalInfoError::None => false,
            TerminalInfoError::Details { .. } => true,
        }
    }
}

/// Holds all terminal info obtained via [`Terminal::info()`]
///
/// [`Terminal::info()`]: crate::Terminal::info
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub struct TerminalInfo {
    pub background: Color,
    pub foreground: Color,
    pub is_dark: bool,
    pub is_light: bool,
    pub binary_luminance: Value,
    pub wcag_luminance: Value,
    pub is_valid: bool,
    pub error: TerminalInfoError,
}

impl TerminalInfo {
    pub fn is_valid(&self) -> bool {
        self.error.is_none()
    }
    pub fn invalid(layer: Layer, error: Error) -> TerminalInfo {
        let is_valid = false;

        let error = TerminalInfoError::Details { layer, message: error.to_string() };

        let background = Color::default_for_bg();
        let foreground = Color::default_for_fg();
        let is_dark = bool::default();
        let is_light = bool::default();
        let binary_luminance = Value::default();
        let wcag_luminance = Value::default();
        TerminalInfo {
            background,
            foreground,
            is_dark,
            is_light,
            binary_luminance,
            wcag_luminance,
            is_valid,
            error,
        }
    }
}
