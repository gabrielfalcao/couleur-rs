use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

use crate::{Error, Layer, Result};

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

    fn to_possible_strings(&self) -> [String; 1] {
        [self.variant_name_snake().to_string()]
    }
}
