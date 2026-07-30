use crate::{Error, Prefix, Result};
use serde::Serialize;

pub trait ToAnsi: Sized + Serialize {
    fn as_ansi_suffix(&self) -> String;
    fn to_ansi(&self) -> String {
        self.to_ansi_with_prefix(None)
    }
    fn to_ansi_with_prefix(&self, prefix: Option<Prefix>) -> String {
        format!("{prefix}{suffix}", prefix = prefix.unwrap_or_default(), suffix = self.as_ansi_suffix())
    }
}
