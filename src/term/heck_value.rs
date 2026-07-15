use std::fmt::Display;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};

pub trait HeckPossibleValue: Sized + Clone + Display {
    fn variant_name_pascal(&self) -> &'static str;
    fn variants<'a>() -> &'a [Self] where Self: Sized;

    fn variant_name_snake(&self) -> String {
        self.variant_name_pascal().to_snake_case()
    }
    fn variant_name_kebab(&self) -> String {
        self.variant_name_pascal().to_kebab_case()
    }
    fn variant_name_train(&self) -> String {
        self.variant_name_pascal().to_train_case()
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

impl ValueEnum for dyn HeckPossibleValue {
    fn value_variants<'a>() -> &'a [Self] {
        Self::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_snake())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Self, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for (variant, possible_strings) in HeckPossibleValue::variants()
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
