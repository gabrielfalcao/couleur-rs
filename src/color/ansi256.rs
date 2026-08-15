use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    str::FromStr,
};

use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
    de::{Error as SerdeError, Visitor},
};
use thiserror::Error as ThisError;

use super::{BLACK, WHITE};
use crate::{
    Contrast,
    Error,
    HEX_RGB_REGEX,
    Layer,
    Prefix,
    Result,
    Terminal,
    ToAnsiEscSuffix,
    Value,
    max_rgb,
    min_rgb,
};

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct Ansi256Color(
    pub code: u8,
);
impl Ansi256Color {
    pub fn new<T: Copy + Into<u8>>(code: T) -> Result<Ansi256Color> {
        Ok(Ansi256Color {
            code: Into::<u8>::into(code)
        })
    }

}
}
impl std::fmt::Display for Ansi256Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_rgb_hex())
    }
}

impl Hash for Ansi256Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_triple().hash(state);
        self.to_rgb_hex().hash(state);
    }
}
impl ToAnsiEscSuffix for Ansi256Color {
    fn to_ansi_esc_suffix(&self) -> String {
        let triple = self.to_triple().iter().map(|v| v.to_string()).collect::<Vec<String>>();
        let color = triple.join(";");
        format!("2;{color}m")
    }
}


#[cfg(test)]
mod test {
    use crate::{Ansi256Color, Result};
    #[test]
    fn test_to_string() -> Result<()> {
        Ok(())
    }
    #[test]
    fn test_get_binary_luminance() -> Result<()> {

        Ok(())
    }
}
