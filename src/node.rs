use serde::{Deserialize, Serialize};

use crate::{
    AnsiRenderable,
    Color,
    ColorPalette,
    Layer,
    Prefix,
    PrefixContainer,
    RenderableColor,
    Reset,
    ToAnsiEscSuffix,
    Wrap,
};

#[derive(Clone, Debug, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum Node {
    Color(Color),
    Reset(Reset),
    Layer(Layer),
    Renderable(RenderableColor),
}
// impl Node {
//     pub fn new<T: Into<Prefix>>(prefix: T) -> Node {
//         Node { prefix: Some(prefix.into()) }
//     }
// }

// impl ToAnsiEscSuffix for Node {
//     fn to_ansi_esc_suffix(&self) -> String {
//         "0m".to_string()
//     }
// }
// impl AnsiRenderable for Node {
//     fn prefix(&self) -> String {
//         self.prefix.unwrap_or_default().to_string()
//     }
// }

// impl Display for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self.to_ansi_esc_suffix())
//     }
// }
