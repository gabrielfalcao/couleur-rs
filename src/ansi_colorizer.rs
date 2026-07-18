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
    pub fn colors(&self) -> Result<(Option<Color>, Option<Color>)> {
        let bg = match self.bg {
            Some(bg) => Some(bg),
            None => match Color::default_for_bg() {
                Ok(bg) => Some(bg),
                Err(_) => None
            }
        };
        let fg = match self.fg {
            Some(fg) => Some(fg),
            None => match Color::default_for_fg() {
                Ok(fg) => Some(fg),
                Err(_) => None
            }
        };
        Ok((bg, fg))
    }
    pub fn colorize<T: std::fmt::Display>(&self, text: T) -> Result<String> {
        let (bg, fg) = self.colors()?;
        let (bg, fg)=if bg.is_none() && fg.is_none() {
            return Err(Error::RenderError(format!("AnsiColorizer requires at least some bg or some fg, but neither was provided")))
        } else if bg.is_none() {
            let fg = fg.unwrap();
            let bg = self.contrast.apply(fg, Layer::BG)?;
            (bg, fg)
        } else if fg.is_none() {
            let bg = bg.unwrap();
            let fg = self.contrast.apply(bg, Layer::FG)?;
            (bg, fg)
        } else {
            let bg = bg.unwrap();
            let fg = fg.unwrap();
            (bg, fg)
        };
        let bg = bg.to_ansi(Layer::BG, self.bold);
        let fg = fg.to_ansi(Layer::FG, self.bold);
        let result = format!("\x1b[0m{bg}{fg}{text}\x1b[0m");
        Ok(result)
    }
}
