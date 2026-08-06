use crate::Layer;
use serde::{Deserialize, Serialize};
use tracing::{Level, event, instrument, span};

/// Holds error details which may occur while querying terminal colors via [`Terminal::background_color`] or [`Terminal::foreground_color`].
#[derive(Clone, Debug, PartialOrd, PartialEq, Eq, Ord, Serialize, Deserialize)]
pub enum TerminalInfoError {
    None,
    Details { message: String },
}
impl TerminalInfoError {
    #[instrument]
    pub fn is_none(&self) -> bool {
        match self {
            TerminalInfoError::None => true,
            TerminalInfoError::Details { .. } => false,
        }
    }
    #[instrument]
    pub fn has_details(&self) -> bool {
        match self {
            TerminalInfoError::None => false,
            TerminalInfoError::Details { .. } => true,
        }
    }
}
