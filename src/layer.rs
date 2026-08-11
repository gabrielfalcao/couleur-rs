use std::fmt::Display;

use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use serde::{Deserialize, Serialize};
#[cfg(feature = "tracing")] use tracing::{Level, event, instrument, span};

use crate::{Color, Error, Result, ToAnsiEscSuffix};

/// Represents the concept of "background" and "foreground" colors in a terminal
#[derive(Clone, Copy, Debug, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layer {
    #[default]
    FG,
    BG,
}
impl Layer {
    #[cfg_attr(feature = "tracing", instrument)]
    pub fn default_color(self) -> Color {
        Color::default_for_layer(self)
    }

    #[cfg_attr(feature = "tracing", instrument)]
    pub fn inverted(self) -> Layer {
        match self {
            Layer::BG => Layer::FG,
            Layer::FG => Layer::BG,
        }
    }

    #[cfg_attr(feature = "tracing", instrument)]
    pub fn code(self) -> i32 {
        let code = match self {
            Layer::BG => 48,
            Layer::FG => 38,
        };
        #[cfg(feature = "tracing")]
        span!(Level::TRACE, "code", code);
        code
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}
impl ToAnsiEscSuffix for Layer {
    fn to_ansi_esc_suffix(&self) -> String {
        let code = self.code();
        format!("{code};")
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

    pub fn variant_name_kebab(&self) -> String {
        self.variant_name_snake().to_kebab_case()
    }

    pub fn variant_name_pascal(&self) -> String {
        self.variant_name_snake().to_pascal_case()
    }

    pub fn variant_name_train(&self) -> String {
        self.variant_name_snake().to_train_case()
    }

    fn to_possible_strings(&self) -> [String; 4] {
        [
            self.variant_name_snake().to_string(),
            self.variant_name_kebab(),
            self.variant_name_pascal(),
            self.variant_name_train(),
        ]
    }
}

impl ValueEnum for Layer {
    fn value_variants<'a>() -> &'a [Layer] {
        Layer::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Layer, String> {
        let val = if ignore_case { val.to_lowercase() } else { val.to_string() };
        let val = val.trim();
        for (variant, possible_strings) in
            Layer::variants().iter().map(|variant| (variant, variant.to_possible_strings()))
        {
            for pos in possible_strings {
                if pos == val {
                    return Ok(*variant);
                }
            }
        }
        return Err(val.to_string());
    }
}
