use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use std::fmt::Display;

#[derive(Clone, Copy, Debug, Default)]
pub enum Reset {
    Before,
    #[default]
    After,
    Around,
    None,
}
impl Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}

impl Reset {
    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            Reset::Before => "before",
            Reset::After => "after",
            Reset::Around => "around",
            Reset::None => "none",
        }
    }
    pub fn variants<'a>() -> &'a [Reset] {
        &[Reset::Before, Reset::After, Reset::Around, Reset::None]
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

impl ValueEnum for Reset {
    fn value_variants<'a>() -> &'a [Reset] {
        Reset::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Reset, String> {
        let val = if ignore_case {
            val.to_lowercase()
        } else {
            val.to_string()
        };
        let val = val.trim();
        for (variant, possible_strings) in Reset::variants()
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
//impl PartialEq<&Reset> for Reset {
//    fn eq(&self, other: &Reset) -> bool {
//        self == *other
//    }
//}
//
//impl PartialOrd<&Reset> for Reset {
//    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
//        self.partial_cmp(*other)
//    }
//}
//
