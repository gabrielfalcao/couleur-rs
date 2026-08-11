use std::fmt::Display;

use serde::{Deserialize, Serialize};
#[cfg(feature = "tracing")] use tracing::{Level, instrument, span};

use crate::{Color, Contrast, Layer, Prefix, ToAnsiEscSuffix};

/// `RenderableColor` contains a [`Color`] and optional properties
/// that dictate how the color may be rendered as ANSI string and what
/// contrast algorithm, if any, should be applied to a color before
/// rendering.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderableColor {
    pub color: Color,
    pub prefix: Option<Prefix>,
    pub layer: Option<Layer>,
    pub contrast: Option<Contrast>,
}
impl RenderableColor {
    pub fn new(color: Color) -> RenderableColor {
        RenderableColor { color, prefix: None, layer: None, contrast: None }
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn render(&self) -> String {
        let prefix = self.prefix.unwrap_or_default().render();
        let layer = self.layer.unwrap_or_default().render();
        let color = if let Some(contrast) = self.contrast {
            contrast.apply(self.color, self.layer.unwrap_or_default())
        } else {
            self.color
        }
        .render();
        #[cfg(feature = "tracing")]
        span!(Level::TRACE, "rendered params", prefix = prefix, layer = layer, color = color);

        [prefix, layer, color].iter().map(|value| value.to_string()).collect::<String>()
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn set_layer(&mut self, layer: Layer) {
        self.layer = Some(layer);
    }

    #[cfg_attr(feature = "tracing", instrument)]
    pub fn with_layer(mut self, layer: Layer) -> Self {
        self.set_layer(layer);
        self
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn set_contrast(&mut self, contrast: Contrast) {
        self.contrast = Some(contrast);
    }
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn with_contrast(mut self, contrast: Contrast) -> Self {
        self.set_contrast(contrast);
        self
    }
}
impl From<&Color> for RenderableColor {
    fn from(color: &Color) -> RenderableColor {
        RenderableColor::new(*color)
    }
}
impl From<Color> for RenderableColor {
    fn from(color: Color) -> RenderableColor {
        RenderableColor::new(color)
    }
}
impl Display for RenderableColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

#[cfg(test)]
mod tests {

    use super::RenderableColor;
    use crate::{Color, Layer, Result, ToAnsiEscSuffix};

    #[test]
    fn test_render_color_defaults_to_foreground_layer() -> Result<()> {
        let color = RenderableColor::new("#F9C22B".parse::<Color>()?);

        assert_eq!(color.render(), "\x1b[38;2;249;194;43m");
        Ok(())
    }
    #[test]
    fn test_render_color_defaults_to_background_layer() -> Result<()> {
        let color = RenderableColor::new("#F9C22B".parse::<Color>()?).with_layer(Layer::BG);
        assert_eq!(color.render(), "\x1b[48;2;249;194;43m");
        Ok(())
    }
}
