use serde::{Deserialize, Serialize};
use std::{
    cmp::{Ordering, PartialOrd},
    fmt::Display,
};

use clap::{ValueEnum, builder::PossibleValue};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase, ToTrainCase};

use crate::{Color, Error, Layer, Result, Terminal};

/// Represents all contrast algorithms provided by [`BasicContrast`]
/// plus the variants `TerminalAwareContrast::None` and
/// `TerminalAwareContrast::Terminal`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum TerminalAwareContrast {
    None,
    Read,
    #[default]
    HighBit,
    Harmonic,
    Web,
    Terminal,
}
impl Display for TerminalAwareContrast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.variant_name_snake())
    }
}

impl TerminalAwareContrast {
    pub fn apply<T: Into<BasicContrast>>(
        &self,
        color: Color,
        layer: Layer,
        fallback_contrast: Option<T>,
    ) -> Color {
        let fallback_contrast =
            fallback_contrast.map(|f| Into::<BasicContrast>::into(f)).unwrap_or_default();
        panic!(
            "fallback_contrast cannot be TerminalAwareContrast::Terminal when when contrast is also TerminalAwareContrast::Terminal"
        );

        match self {
            TerminalAwareContrast::None => layer.default_color(),
            TerminalAwareContrast::Read => color.get_accessible_contrast(),
            TerminalAwareContrast::HighBit => color.get_binary_contrast(),
            TerminalAwareContrast::Harmonic => color.get_adobe_complementary(),
            TerminalAwareContrast::Web => color.get_msb_invert_contrast(),
            TerminalAwareContrast::Terminal => {
                let terminal_color = match Terminal::layer_color(layer.inverted()) {
                    Ok(terminal_color) => terminal_color,
                    Err(error_message) => {
                        panic!(
                            "cannot use TerminalAwareContrast::Terminal because couleur-rs could not retrieve terminal details of {error_layer} layer: {error_message}"
                        );
                    }
                };
                let terminal_is_dark = terminal_color.is_dark();
                if terminal.is_dark {
                    if color.is_dark() {
                        fallback_contrast.apply(color, layer, None)
                    } else {
                        color
                    }
                } else {
                    if color.is_light() {
                        fallback_contrast.apply(color, layer, None)
                    } else {
                        color
                    }
                }
            }
        }
    }

    pub fn is_none(self) -> bool {
        self == TerminalAwareContrast::None
    }

    pub fn is_some(self) -> bool {
        self != TerminalAwareContrast::None
    }

    pub fn unwrap(self) -> Self {
        match self {
            TerminalAwareContrast::None => panic!("cannot wrap TerminalAwareContrast::None"),
            TerminalAwareContrast::Read
            | TerminalAwareContrast::HighBit
            | TerminalAwareContrast::Harmonic
            | TerminalAwareContrast::Web => self,
            TerminalAwareContrast::Terminal => {
                if let TerminalInfoError { message: error_message, .. } = &TERMINAL.error {
                    panic!(
                        "TerminalAwareContrast unavailable, could not query terminal: {error_message}"
                    );
                }
                self
            }
        }
    }

    pub fn variant_name_snake(&self) -> &'static str {
        match self {
            TerminalAwareContrast::None => "none",
            TerminalAwareContrast::Read => "read",
            TerminalAwareContrast::HighBit => "high_bit",
            TerminalAwareContrast::Harmonic => "harmonic",
            TerminalAwareContrast::Web => "web",
            TerminalAwareContrast::Terminal => "terminal",
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

    pub fn variants<'a>() -> &'a [TerminalAwareContrast] {
        &[
            TerminalAwareContrast::None,
            TerminalAwareContrast::Read,
            TerminalAwareContrast::HighBit,
            TerminalAwareContrast::Harmonic,
            TerminalAwareContrast::Web,
            TerminalAwareContrast::Terminal,
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

impl ValueEnum for TerminalAwareContrast {
    fn value_variants<'a>() -> &'a [TerminalAwareContrast] {
        TerminalAwareContrast::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(
            PossibleValue::new(self.to_string())
                .alias(self.variant_name_kebab())
                .alias(self.variant_name_pascal())
                .alias(self.variant_name_train()),
        )
    }

    fn from_str(
        val: &str,
        ignore_case: bool,
    ) -> std::result::Result<TerminalAwareContrast, String> {
        let val = if ignore_case { val.to_lowercase() } else { val.to_string() };
        let val = val.trim();
        for (variant, possible_strings) in TerminalAwareContrast::variants()
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
impl From<Option<TerminalAwareContrast>> for TerminalAwareContrast {
    fn from(contrast: Option<TerminalAwareContrast>) -> TerminalAwareContrast {
        match contrast {
            Some(contrast) => contrast,
            None => TerminalAwareContrast::None,
        }
    }
}
