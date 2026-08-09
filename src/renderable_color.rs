#[cfg(feature = "tracing")] use tracing::{Level, event, instrument, span};
use {
    crate::{AnsiRenderable, Color, Contrast, Error, Exit, Layer, Prefix, Reset, Result, Wrap},
    serde::{Deserialize, Serialize},
    std::fmt::Display,
};
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
    fn render(&self) -> String{
        let mut parts = Vec::<Color>::new();
        let prefix = prefix.unwrap_or_default().render();
        let layer = layer.unwrap_or_default().render();
        parts.push(prefix);
        parts.push(layer);
        parts.push(if let Some(contrast) = contrast.clone() {
            contrast.apply(color,layer).render()
        } else {
            color.render()
        });
        let parts = [prefix, layer, color].iter().collect::(String);


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
