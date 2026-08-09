#[cfg(feature = "tracing")] use tracing::{Level, event, instrument, span};
use {
    crate::{AnsiRenderable, Color, Contrast, Error, Exit, Layer, Prefix, Reset, Result, Wrap},
    bon::Builder,
    serde::{Deserialize, Serialize},
    std::fmt::Display,
};
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize, Builder)]
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
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn set_color(&mut self, value: Color) {
        self.color = value;
        self.color
    }
    pub fn with_color(mut self, value: Color) -> RenderableColor {
        self.set_color(value);
        self
    }
    pub fn prefix(&self) -> Option<Prefix> {
        self.prefix
    }
    pub fn set_prefix(&mut self, value: Prefix) {
        self.prefix = Some(value);
    }
    pub fn with_prefix(mut self, value: Prefix) -> RenderableColor {
        self.set_prefix(value);
        self
    }
    pub fn layer(&self) -> Option<Layer> {
        self.layer
    }
    pub fn set_layer(&mut self, value: Layer) {
        self.layer = Some(value);
        self.layer
    }
    pub fn with_layer(mut self, value: Layer) -> RenderableColor {
        self.set_layer(value);
        self
    }

    pub fn contrast(&self) -> Option<Contrast> {
        self.contrast
    }
    pub fn set_contrast(&mut self, value: Contrast) -> Option<Contrast> {
        self.contrast = Some(value);
    }
    pub fn with_contrast(mut self, value: Contrast) -> RenderableColor {
        self.set_contrast(value);
        self
    }

    pub fn variant(&self) -> String {
        format!("RenderableCor(serde_yaml.to_string(&self))")
    }
}
impl Display for RenderableColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}
impl AnsiRenderable for RenderableColor {
    fn render(&self) -> String {
        let mut parts = Vec::<String>::new();
        let prefix = self.prefix.unwrap_or_default().render();
        let layer = self.layer.unwrap_or_default().render();
        parts.push(prefix);
        parts.push(layer);
        parts.push(if let Some(contrast) = self.contrast.clone() {
            contrast.apply(self.color, self.layer.unwrap_or_default()).render()
        } else {
            self.color.render()
        });
        let result = parts.into_iter().collect::<String>();
        Ok(result)
    }
}
// impl Display for RenderableColor {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let color = self.color;
//         let prefix = self.prefix;
//         let prefix = self.prefix;
//         let contrast = self.contrast;
//         let mut parts = vec![format!("color:{color}")];
//         if let Some(prefix) = &self.prefix {
//             parts.push(prefix.to_string())
//         }
//
//         if let Some(color) = &self.color {
//             parts.push(color.to_string())
//         }
//         if let Some(layer) = &self.layer {
//             parts.push(layer.to_string())
//         }
//
//         if let Some(contrast) = &self.contrast {
//             parts.push(contrast.to_string())
//         }
//         let write = parts.join(",");
//         write!(f, "{write}")
//     }
// }
