use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{Error, Result};

/// Represents the concept of "background" and "foreground" colors in a terminal
#[derive(Clone, Copy, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layer {
    #[default]
    FG,
    BG,
}
impl Layer {
    pub fn inverted(self) -> Layer {
        match self {
            Layer::BG => Layer::FG,
            Layer::FG => Layer::BG,
        }
    }

    pub fn code(self) -> i32 {
        match self {
            Layer::BG => 48,
            Layer::FG => 38,
        }
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}

impl Layer {
    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            Layer::BG => "bg",
            Layer::FG => "fg",
        }
    }

    pub fn variants<'a>() -> &'a [Layer] {
        &[Layer::BG, Layer::FG]
    }

    fn to_possible_strings(&self) -> [String; 1] {
        [self.variant_name_snake().to_string()]
    }
}
