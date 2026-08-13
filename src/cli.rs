use clap::Parser;

use crate::{Prefix, set_runtime_prefix};

#[derive(Parser, Debug, Clone)]
pub struct SharedRenderingOpts {
    #[arg(
        long,
        help = "automatically adds a reset code after rendering colored text",
        action = clap::ArgAction::SetTrue,
        default_value = "false",
        conflicts_with = "no_auto_reset"
    )]
    pub auto_reset: bool,
    #[arg(long, action = clap::ArgAction::SetFalse, default_value="true", help = "does not reset colors after rendering", conflicts_with = "auto_reset")]
    pub no_auto_reset: bool,

    #[arg(short, long, help = "the ANSI sequence prefix", default_value = "hex")]
    pub prefix: Prefix,

    #[arg(skip)]
    initialized: bool,
}
impl SharedRenderingOpts {
    pub fn init(&mut self) {
        set_runtime_prefix(self.prefix);
        self.initialized = true;
    }
    pub fn prefix(&self) -> Prefix {
        if !self.initialized {
            todo!("SharedRenderingOpts must be initialized with .init() ");
        }
        self.prefix
    }
    pub fn dont_append(&self) -> bool {
        if !self.initialized {
            todo!("SharedRenderingOpts must be initialized with .init() ");
        }
        self.no_auto_reset || !self.auto_reset
    }

    pub fn add_reset_to_last_node(&self) -> bool {
        if !self.initialized {
            todo!("SharedRenderingOpts must be initialized with .init() ");
        }
        self.auto_reset || !self.no_auto_reset
    }
}
