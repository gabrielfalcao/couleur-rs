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
        let error = match self {
            Error::RenderableColor(value) => value.to_string(),
            Error::IOError(value) => value.to_string(),
            Error::RuntimeError(value) => value.to_string(),
            Error::ConversionToU8Error(value) => value.to_string(),
            Error::TerminalQueryError(value) => value.to_string(),
            Error::RenderError(value) => value.to_string(),
        };
        let variant = self.variant();
        write!(f, "{variant}: {error}")
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
