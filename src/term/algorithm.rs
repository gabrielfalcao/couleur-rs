use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Algorithm {
    Read,
    HighBit,
    Harmonic,
    Web,
}
impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name(),)
    }
}

impl Algorithm {
    pub fn variant_name(&self) -> &'static str {
        match self {
            Algorithm::Read => "Read",
            Algorithm::HighBit => "HighBit",
            Algorithm::Harmonic => "Harmonic",
            Algorithm::Web => "Web",
        }
    }
    pub fn variant_name_snake(&self) -> String {
        self.variant_name().to_snake_case()
    }
    pub fn variant_name_kebab(&self) -> String {
        self.variant_name().to_kebab_case()
    }
    pub fn variant_name_pascal(&self) -> String {
        self.variant_name().to_pascal_case()
    }
    pub fn variant_name_train(&self) -> String {
        self.variant_name().to_train_case()
    }

    pub fn variants<'a>() -> &'a [Algorithm] {
        &[
            Algorithm::Read,
            Algorithm::HighBit,
            Algorithm::Harmonic,
            Algorithm::Web,
        ]
    }
    fn to_possible_strings(&self) -> [String; 5] {
        [
            self.variant_name().to_string(),
            self.variant_name_snake(),
            self.variant_name_kebab(),
            self.variant_name_pascal(),
            self.variant_name_train(),
        ]
    }
}

impl ValueEnum for Algorithm {
    fn value_variants<'a>() -> &'a [Algorithm] {
        Algorithm::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name())
                .alias(self.variant_name_snake())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Algorithm, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for (variant, possible_strings) in Algorithm::variants()
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
