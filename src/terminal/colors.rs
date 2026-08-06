use crate::{Color, impl_struct_representative_of_color};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct TerminalBackground {
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct TerminalForeground {
    pub color: Color,
}

impl_struct_representative_of_color!(TerminalBackground, color);
impl_struct_representative_of_color!(TerminalForeground, color);
