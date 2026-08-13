use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{AnsiRenderable, Prefix, ToAnsiEscSuffix};

/// Represents the ansi code `[0m` without any [ansi escape](https://en.wikipedia.org/wiki/C0_and_C1_control_codes#ESC) prefix
#[derive(Clone, Debug, Copy, Default, PartialOrd, Ord, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reset {
    pub prefix: Option<Prefix>,
}
impl Reset {
    pub fn new<T: Into<Prefix>>(prefix: T) -> Reset {
        Reset { prefix: Some(prefix.into()) }
    }
}

impl ToAnsiEscSuffix for Reset {
    fn to_ansi_esc_suffix(&self) -> String {
        "[0m".to_string()
    }
}
impl AnsiRenderable for Reset {
    fn prefix(&self) -> String {
        self.prefix.unwrap_or_default().to_string()
    }
}

impl Display for Reset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_ansi_esc_suffix())
    }
}
