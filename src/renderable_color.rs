use crate::{AnsiRenderable, Color, Contrast, Error, Exit, Layer, Prefix, Reset, Result, Wrap};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tracing")] use tracing::{Level, event, instrument, span};

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderableColor {
    pub(crate) color: Color,
    pub(crate) prefix: Option<Prefix>,
    pub(crate) layer: Option<Layer>,
    pub(crate) contrast: Option<Contrast>,
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
impl From<Color> for RenderableColor {
    fn from(color: Color) -> RenderableColor {
        RenderableColor::new(color)
    }
}
#[cfg(test)]
mod tests {
    use super::RenderableColor;
    use crate::{AnsiRenderable, Color, Contrast, Error, Layer, Result, global_setup};
    use std::str::FromStr;

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
