use crate::{Color, Error, Layer, Result, Value};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, marker::PhantomData};
use terminal_colorsaurus::{QueryOptions, background_color, foreground_color};
use tracing::{Level, event, instrument, span};

pub(crate) mod error;
pub use error::TerminalInfoError;

/// Holds all terminal info obtained via [`Terminal::info()`]
///
/// [`Terminal::info()`]: crate::Terminal::info
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub struct TerminalInfo {
    pub background: TerminalBackground,
    pub foreground: TerminalForeground,
    pub is_dark: bool,
    pub is_light: bool,
    pub binary_luminance: Value,
    pub wcag_luminance: Value,
    pub is_valid: bool,
    pub error: TerminalInfoError,
}

impl TerminalInfo {
    #[instrument]
    pub fn is_valid(&self) -> bool {
        self.error.is_none()
    }
    #[instrument]
    pub fn invalid(error: Error) -> TerminalInfo {
        let is_valid = false;

        let error = TerminalInfoError::Details { message: error.to_string() };

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
