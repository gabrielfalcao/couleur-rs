use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use std::fmt::Display;
use crate::term::HeckPossibleValue;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Algorithm {
    Read,
    HighBit,
    Harmonic,
    Web,
}

impl HeckPossibleValue for Algorithm {
 fn variant_name_pascal(&self) -> &'static str {
        match self {
            Algorithm::Read => "Read",
            Algorithm::HighBit => "HighBit",
            Algorithm::Harmonic => "Harmonic",
            Algorithm::Web => "Web",
        }
    }
 fn variants<'a>() -> &'a [Algorithm] {
        &[
            Algorithm::Read,
            Algorithm::HighBit,
            Algorithm::Harmonic,
            Algorithm::Web,
        ]
    }
}
