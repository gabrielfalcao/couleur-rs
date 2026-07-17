use crate::{Contrast, Error, Exit, Layer, Color, Reset, Result, Wrap};

#[derive(Debug, Clone, Copy)]
pub struct AnsiColorizer {
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub bold: bool,
    pub contrast: Contrast,
    pub reset: Reset,
    pub wrap: Wrap,
}

impl AnsiColorizer {
    pub fn colors(&self) -> Result<(Color, Color, Option<Contrast>)> {
        let bg = match self.bg {
            Some(bg) => bg,
            None => Color::default_for_bg()?
        };
        let fg = match self.fg {
            Some(fg) => fg,
            None => Color::default_for_fg()?
        };
        let contrast = self.contrast;
        Ok((bg, fg, contrast.into()))
    }
    pub fn colorize<T: std::fmt::Display>(&self, text: T) -> Result<String> {
        let (bg, fg, contrast) = self.colors()?;
        let bg = bg.to_ansi(Layer::BG, true);
        let fg = fg.to_ansi(Layer::FG, true);
        let result = format!("\x1b[0m{bg}{fg}{text}\x1b[0m");
        Ok(result)
    }
}
