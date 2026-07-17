use crate::{Algorithm, Error, Exit, Layer, Color, Reset, Result, Wrap};

#[derive(Debug, Clone, Copy)]
pub struct Colorizer {
    pub bg: Option<Color>,
    pub fg: Option<Color>,
    pub bold: bool,
    pub contrast: Algorithm,
    pub reset: Reset,
    pub wrap: Wrap,
}

impl Colorizer {
    pub fn colors(&self) -> Result<(Color, Color, Option<Algorithm>)> {
        let (bg, fg, contrast) = if self.fg.is_some() && self.bg.is_some() && self.contrast.is_none() {
            dbg!(self.bg, self.fg);
            (self.bg.unwrap(), self.fg.unwrap(), None)
        } else if self.bg.is_none() && self.contrast.is_some() && self.contrast.is_some() {
            let fg = self.fg.unwrap();
            let bg = fg.contrast(self.contrast.unwrap());
            (bg, self.fg.unwrap(), Some(self.contrast))
        } else if self.fg.is_some() && self.fg.is_none() && self.contrast.is_some() {
            let fg = self.fg.unwrap();
            let bg = fg.contrast(self.contrast.unwrap());
            (bg, fg, Some(self.contrast))
        } else if self.fg.is_none() && self.fg.is_some() && self.contrast.is_some() {
            let bg = self.bg.unwrap();
            let fg = bg.contrast(self.contrast.unwrap());
            (bg, fg, Some(self.contrast))
        } else if self.fg.is_none() && self.bg.is_none() && self.contrast.is_none() {
            let bg = Color::default_for_bg()?;
            let fg = Color::default_for_fg()?;
            (bg, fg, Some(self.contrast))
        } else if self.fg.is_none() && self.bg.is_none() && self.contrast.is_some() {
            let fg = Color::default_for_fg()?;
            (fg.contrast(self.contrast.unwrap()), fg, Some(self.contrast))
        } else {
            let bg = Color::default_for_bg()?;
            let fg = Color::default_for_fg()?;
            (bg, fg, Some(self.contrast))
        };
        return Ok((bg, fg, contrast));
    }
    pub fn colorize<T: std::fmt::Display>(&self, text: T) -> Result<String> {
        let (bg, fg, contrast) = self.colors()?;
        let bg = bg.to_ansi(Layer::BG, true);
        let fg = fg.to_ansi(Layer::FG, true);
        let result = format!("\x1b[0m{bg}{fg}{text}\x1b[0m");
        Ok(result)
    }
}
