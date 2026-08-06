use crate::{Color, Error, Layer, Result, Value};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, marker::PhantomData, time::Duration};
use terminal_colorsaurus::{
    ColorPalette,
    QueryOptions,
    background_color,
    color_palette,
    foreground_color,
};
use tracing::{Level, event, instrument, span};

/// Terminal is an abstraction to query information regarding the
/// background and foreground colors of the terminal at runtime along
/// with information such as whether the background is dark or light
/// and the binary luminance of both. The [method
/// used](terminal_colorsaurus::color_palette) for achieving this is
/// not guaranteed to work with all terminal emulators and also does
/// not work then none of the stdio file descriptors are a TTY.
///
///
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
///
/// ## Caveats
///
/// ### Latency
///
/// NOTE that **each and every associated function** in this
/// [`Terminal`] struct performs one query to the terminal, and every
/// query to the terminal adds latency that might peak at around 400
/// milliseconds. (The maximum timeout is configurable via
/// [`RuntimeSettings::set_query_timeout()`])
///
/// This issue is mitigated by this crate by querying the terminal
/// only once during library initialization and storing the result in
/// the static variable [`couleur::TERMINAL`] but you should be aware
/// of this behavior and know that calling any of these functions
/// several times will slow down your application severely.
///
/// When in doubt [`couleur::TERMINAL`] instead!
///
/// ### When Terminal Querying Fails
///
/// This library is designed in such a way that the concept of
/// terminal background and foreground colors falls back to values
/// configurable during runtime. Therefore it is up to you to decide
/// what to do then the terminal querying fails.
///
/// The default setting for fallbacks is BLACK for background and
/// WHITE for foreground colors, but you can override the global
/// setting (which is stored in the static variable
/// [`couleur::SETTINGS`] by calling
/// [`RuntimeSettings::set_fallback_background_color`] and  [`RuntimeSettings::set_fallback_foreground_color`] respectively which will cause methods of structs and enums along the crate such as [`Color::default_for_bg()`] and  [`Color::default_for_fg()`] to use the cached result of terminal query when the startup query succeeded and fallback to configured values when that failed.
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize, Default)]
pub struct Terminal;
impl Terminal {
    pub const MINIMUM_QUERY_TIMEOUT: Duration = Duration::from_millis(337);

    /// returns the background color of the [`Terminal`]
    #[instrument]
    pub fn background_color() -> Result<Color> {
        let terminal_bg_color = background_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }

    /// returns the foreground color of the [`Terminal`]
    #[instrument]
    pub fn foreground_color() -> Result<Color> {
        let terminal_bg_color = foreground_color(QueryOptions::default())?;
        let (r, g, b) = terminal_bg_color.scale_to_8bit();
        Ok(Color::from_triple(r.into(), g.into(), b.into()))
    }

    #[instrument]
    pub fn layer_color(layer: Layer) -> Result<Color> {
        Ok(match layer {
            Layer::BG => Self::background_color()?,
            Layer::FG => Self::foreground_color()?,
        })
    }

    #[instrument]
    pub fn background_luminance() -> Result<Value> {
        let background_color = Self::background_color()?;
        Ok(background_color.get_binary_luminance().into())
    }

    #[instrument]
    pub fn foreground_luminance() -> Result<Value> {
        let foreground_color = Self::foreground_color()?;
        Ok(foreground_color.get_binary_luminance().into())
    }

    #[instrument]
    pub fn is_dark() -> Result<bool> {
        let luminance = Self::background_luminance()?;
        Ok(luminance < 128.0)
    }

    #[instrument]
    pub fn is_light() -> Result<bool> {
        let luminance = Self::background_luminance()?;
        Ok(luminance >= 128.0)
    }

    #[instrument]
    pub fn new(background: Color, foreground: Color) -> TerminalInfo {
        let is_dark = background.is_dark();
        let is_light = background.is_light();
        let binary_luminance = background.get_binary_luminance();
        let wcag_luminance = background.get_wcag_luminance();
        TerminalInfo {
            background,
            foreground,
            is_dark,
            is_light,
            binary_luminance,
            wcag_luminance,
            is_valid: true,
            error: TerminalInfoError::None,
        }
    }

    /// returns a [`TerminalInfo`] by querying the terminal's
    /// background and foreground colors and computing their luminance
    /// to obtain all the details in [`TerminalInfo`].
    ///
    /// Shortcut to calling [`Terminal::query_info(Terminal::MINIMUM_QUERY_TIMEOUT)`]
    #[instrument]
    pub fn info() -> TerminalInfo {
        TerminalInfo::query_info(Terminal::MINIMUM_QUERY_TIMEOUT)
    }

    /// queries the terminal background and foreground colors,
    /// returning a [`TerminalInfo`] based on the result.
    #[instrument]
    pub fn query_info(query_timeout: Duration) -> TerminalInfo {
        let (background, foreground) =
            match color_palette(terminal_colorsaurus::QueryOptions { timeout: query_timeout }) {
                Ok(ColorPalette { background, foreground }) => {
                    TerminalInfo::new(background.into(), foreground.into())
                }

                Err(error) => TerminalInfo::invalid(error.into()),
            };
    }
}
