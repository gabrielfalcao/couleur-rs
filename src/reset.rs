use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::ToAnsiEscSuffix;

/// Represents the ansi code `[0m` without any [ansi escape](https://en.wikipedia.org/wiki/C0_and_C1_control_codes#ESC) prefix
#[derive(Clone, Debug, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reset;

impl ToAnsiEscSuffix for Reset {
    fn to_ansi_esc_suffix(&self) -> String {
        "[0m".to_string()
    }
}

impl Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self}")
    }
}
