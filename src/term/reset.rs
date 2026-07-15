use crate::{RGBColor, term::HeckPossibleValue};
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
    fn variant_name_pascal(&self) -> &'static str {
        match self {
            Reset::Before => "Before",
            Reset::After => "After",
            Reset::Around => "Around",
            Reset::None => "None",
        }
    }
    fn variants<'a>() -> &'a [Reset] {
        &[Reset::Before, Reset::After, Reset::Around, Reset::None]
    }
}
