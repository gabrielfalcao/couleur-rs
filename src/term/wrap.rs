use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default)]
pub enum Wrap {
    #[default]
    Before,
    After,
    Around,
}

impl Display for Wrap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}

impl Wrap {
    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            Wrap::Before => "before",
            Wrap::After => "after",
            Wrap::Around => "around",
        }
    }
    pub fn variants<'a>() -> &'a [Wrap] {
        &[Wrap::Before, Wrap::After, Wrap::Around]
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

impl ValueEnum for Wrap {
    fn value_variants<'a>() -> &'a [Wrap] {
        Wrap::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Wrap, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for (variant, possible_strings) in Wrap::variants()
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
//impl PartialEq<&Wrap> for Wrap {
//    fn eq(&self, other: &Wrap) -> bool {
//        self == *other
//    }
//}
//
//impl PartialOrd<&Wrap> for Wrap {
//    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
//        self.partial_cmp(*other)
//    }
//}
//
