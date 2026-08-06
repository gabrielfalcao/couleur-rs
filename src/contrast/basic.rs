use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};

use crate::{Color, Error, Layer, Result, TERMINAL};

/// Set of contrast algorithms applicable to [`Color`]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum BasicContrast {
    Read,
    #[default]
    HighBit,
    Harmonic,
    Web,
}
impl Display for BasicContrast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}

impl BasicContrast {
    pub fn apply(&self, color: Color) -> Color {
        match self {
            BasicContrast::Read => color.get_accessible_contrast(),
            BasicContrast::HighBit => color.get_binary_contrast(),
            BasicContrast::Harmonic => color.get_adobe_complementary(),
            BasicContrast::Web => color.get_msb_invert_contrast(),
        }
    }

    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            BasicContrast::Read => "read",
            BasicContrast::HighBit => "high_bit",
            BasicContrast::Harmonic => "harmonic",
            BasicContrast::Web => "web",
        }
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

    pub fn variants<'a>() -> &'a [BasicContrast] {
        &[BasicContrast::Read, BasicContrast::HighBit, BasicContrast::Harmonic, BasicContrast::Web]
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

impl ValueEnum for BasicContrast {
    fn value_variants<'a>() -> &'a [BasicContrast] {
        BasicContrast::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<BasicContrast, String> {
        let val = if ignore_case { val.to_lowercase() } else { val.to_string() };
        let val = val.trim();
        for (variant, possible_strings) in
            BasicContrast::variants().iter().map(|variant| (variant, variant.to_possible_strings()))
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
impl From<Option<BasicContrast>> for BasicContrast {
    fn from(contrast: Option<BasicContrast>) -> BasicContrast {
        match contrast {
            Some(contrast) => contrast,
            None => BasicContrast::None,
        }
    }
}
