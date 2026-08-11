use std::fmt::{Debug, Display};

use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};
use serde::{Deserialize, Serialize};

use crate::ToAnsiEscSuffix;

/// Represents an [ANSI Escape
/// character](https://en.wikipedia.org/wiki/Escape_character#ASCII_escape_character)
/// which prefixes ANSI sequences rendered by elements of this crate
/// such as [`RenderableColor`](crate::RenderableColor)
///
/// The supported escape characters are available as follows:
///
/// - [`Octal`](crate::Prefix::Octal) => `\033`
/// - [`Hex`](crate::Prefix::Hex) => `\x1b`
/// - [`Unicode`](crate::Prefix::Unicode) => `\u{1b}`
/// - [`Escape`](crate::Prefix::Escape) => `\E`
///
/// # Rendering Support
///
/// At least one of each variant of [`Prefix`](crate::Prefix) is
/// supported by different context of tools such as terminal emulators and shell
/// interpreters which are the scope of this crate.
///
/// This crate focuses primarily in rendering text decorated with
/// colors in terminal emulators that support true color rendering,
/// for this reason the [`Prefix`] enum defaults to the variant
/// [`Hex`](crate::Prefix::Hex)
///
/// The reason to provide other variants such as the
/// [`Octal`](crate::Prefix::Octal) (`\033`) is to allow using this
/// crate to colorize, for example, the bash PS1 variable.
#[derive(Clone, Debug, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum Prefix {
    Octal,
    #[default]
    Hex,
    Unicode,
    Escape,
}
impl Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Prefix::Octal => "\033",
                Prefix::Hex => "\x1b",
                Prefix::Unicode => "\u{1b}",
                Prefix::Escape => r"\E",
            }
        )
    }
}
impl ToAnsiEscSuffix for Prefix {
    fn to_ansi_esc_suffix(&self) -> String {
        format!("{self}[")
    }
}

impl Prefix {
    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            Prefix::Octal => "octal",
            Prefix::Hex => "hex",
            Prefix::Unicode => "unicode",
            Prefix::Escape => "escape",
        }
    }

    pub fn variants<'a>() -> &'a [Prefix] {
        &[Prefix::Octal, Prefix::Hex, Prefix::Escape]
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

impl ValueEnum for Prefix {
    fn value_variants<'a>() -> &'a [Prefix] {
        Prefix::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Prefix, String> {
        let val = if ignore_case { val.to_lowercase() } else { val.to_string() };
        let val = val.trim();
        for (variant, possible_strings) in
            Prefix::variants().iter().map(|variant| (variant, variant.to_possible_strings()))
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
//impl PartialEq<&Prefix> for Prefix {
//    fn eq(&self, other: &Prefix) -> bool {
//        self == *other
//    }
//}
//
//impl PartialOrd<&Prefix> for Prefix {
//    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
//        self.partial_cmp(*other)
//    }
//}
//
