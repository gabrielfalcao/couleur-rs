use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Represents the intent of adding an "ansi reset" sequence before,
/// after or around an ANSI color sequence.
#[derive(Clone, Debug, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub enum Reset {
    Before,
    #[default]
    After,
    Around,
    None,
}

impl Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "reset {}", self.variant_name_snake())
    }
}
impl Reset {
    pub fn code() -> &'static str {
        "[0m"
    }

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

    fn to_possible_strings(&self) -> [String; 1] {
        [self.variant_name_snake().to_string()]
    }
}
