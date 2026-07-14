use crate::RGBColor;
use clap::{ValueEnum, builder::PossibleValue};
use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum Reset {
    Before,
    After,
    Around,
    None,
}

impl HeckPossibleValue for Reset {
    pub fn variant_name(&self) -> &'static str {
        match self {
            Reset::Before => "Before",
            Reset::After => "After",
            Reset::Around => "Around",
            Reset::None => "None",
        }
    }
    pub fn variants<'a>() -> &'a [Reset] {
        &[Reset::Before, Reset::After, Reset::Around, Reset::None]
    }
}
