use crate::{Color, Contrast, Error, Layer, Prefix, Reset, Result, Wrap};
use serde::{Deserialize, Serialize};

/// Utility struct to [`colorize()`] arbitrary text
///
/// [`colorize()`]: crate::AnsiColorizer::colorize
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnsiColorizer {
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub contrast: Contrast,
    pub reset: Reset,
    pub prefix: Option<Prefix>,
    pub wrap: Wrap,
}

impl AnsiColorizer {
    pub fn colors(&self) -> Result<(Option<Color>, Option<Color>)> {
        let bg = match self.bg {
            Some(bg) => Some(bg),
            None => Some(Color::default_for_bg()),
        };
        let fg = match self.fg {
            Some(fg) => Some(fg),
            None => Some(Color::default_for_fg()),
        };
        Ok((bg, fg))
    }

    /// Colorizes any objects which implement [`std::fmt::Display`]
    pub fn colorize<T: std::fmt::Display>(&self, text: T) -> Result<String> {
        let (bg, fg) = self.colors()?;
        let (bg, fg) = if bg.is_none() && fg.is_none() {
            return Err(Error::RenderError(format!(
                "AnsiColorizer requires at least some bg or some fg, but \
                 neither was provided"
            )));
        } else if bg.is_none() {
            let fg = fg.unwrap();
            let bg = self.contrast.apply(fg, Layer::BG);
            (bg, fg)
        } else if fg.is_none() {
            let bg = bg.unwrap();
            let fg = self.contrast.apply(bg, Layer::FG);
            (bg, fg)
        } else {
            let bg = bg.unwrap();
            let fg = fg.unwrap();
            (bg, fg)
        };
        let bg = bg.to_ansi_with_prefix(Layer::BG, self.prefix);
        let fg = fg.to_ansi_with_prefix(Layer::FG, self.prefix);
        let result = format!(
            "{prefix}[0m{bg}{fg}{text}{prefix}[0m",
            prefix = self.prefix.unwrap_or_default()
        );
        Ok(result)
    }
}
