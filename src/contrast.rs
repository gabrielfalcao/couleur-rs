use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
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
    pub fn variant_name_kebab(&self) -> String {
        self.variant_name_snake().to_kebab_case()
    }
    pub fn variant_name_pascal(&self) -> String {
        self.variant_name_snake().to_pascal_case()
    }
    pub fn variant_name_train(&self) -> String {
        self.variant_name_snake().to_train_case()
    }

    pub fn variants<'a>() -> &'a [Contrast] {
        &[
            Contrast::Read,
            Contrast::HighBit,
            Contrast::Harmonic,
            Contrast::Web,
        ]
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

impl ValueEnum for Contrast {
    fn value_variants<'a>() -> &'a [Contrast] {
        Contrast::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Contrast, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for (variant, possible_strings) in Contrast::variants()
            .iter()
            .map(|variant| (variant, variant.to_possible_strings()))
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
impl From<Option<Contrast>> for Contrast {
    fn from(contrast: Option<Contrast>) -> Contrast {
        match contrast {
            Some(contrast) => contrast,
            None => Contrast::None,
        }
    }
}
