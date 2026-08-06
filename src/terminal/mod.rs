pub(crate) mod colors;
pub use colors::{TerminalBackground, TerminalForeground};

pub(crate) mod info;
pub use info::{TerminalInfo, TerminalInfoError};

pub(crate) mod terminal;
pub use terminal::Terminal;
