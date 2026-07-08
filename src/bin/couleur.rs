use clap::{Parser, Subcommand};

use couleur_rs::{Error, Exit, Result};

use couleur_rs::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {
    #[arg(long)]
    bg: Option<String>,

    #[arg(long)]
    fg: Option<String>,

    #[arg(short, long)]
    reset: bool,
}
impl Cli {
    pub fn bg_code(&self) -> String {
        self.bg.as_ref().map(|bg| bg.to_string()).unwrap_or_default()
    }
    pub fn bg(&self, red: u8, green: u8, blue: u8) -> String {
        format!("\x1b[1;48;2;{red};{green};{blue}m")
    }

    pub fn fg_code(&self) -> String {
        self.fg.as_ref().map(|fg| fg.to_string()).unwrap_or_default()
    }
    pub fn fg(&self, red: u8, green: u8, blue: u8) -> String {
        format!("\x1b[1;38;2;{red};{green};{blue}m")
    }
}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let reset = "\x1b[0m";
        println!(
            "{}{} {}{}{reset}",
            &self.fg(225, 184,  86),
            "Hello",
            &self.fg(141, 204, 48),
            "World"
        );

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
