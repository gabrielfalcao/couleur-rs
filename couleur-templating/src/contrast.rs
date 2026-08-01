use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

use crate::{Color, Error, Layer, Result};

/// Set of contrast algorithms applicable to [`Color`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Contrast {
    None,
    Read,
    #[default]
    HighBit,
    Harmonic,
    Web,
}
impl Display for Contrast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}

impl Contrast {
    pub fn apply(&self, color: Color, layer: Layer) -> Color {
        match self {
            Contrast::None => layer.default_color(),
            Contrast::Read => color.get_accessible_contrast(),
            Contrast::HighBit => color.get_binary_contrast(),
            Contrast::Harmonic => color.get_adobe_complementary(),
            Contrast::Web => color.get_msb_invert_contrast(),
        }
    }

    pub fn is_none(self) -> bool {
        self == Contrast::None
    }

    pub fn is_some(self) -> bool {
        self != Contrast::None
    }

    pub fn unwrap(self) -> Self {
        self
    }

    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            Contrast::None => "none",
            Contrast::Read => "read",
            Contrast::HighBit => "high_bit",
            Contrast::Harmonic => "harmonic",
            Contrast::Web => "web",
        }
    }

    pub fn variants<'a>() -> &'a [Contrast] {
        &[Contrast::Read, Contrast::HighBit, Contrast::Harmonic, Contrast::Web]
    }

    fn to_possible_strings(&self) -> [String; 4] {
        [self.variant_name_snake().to_string()]
    }
}
