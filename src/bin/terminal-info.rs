#![allow(unused)]
use {
    clap::Parser,
    couleur_rs::{
        BLACK,
        Color,
        Contrast,
        Error,
        Exit,
        Layer,
        Prefix,
        Reset,
        Result,
        TERMINAL,
        Terminal,
        WHITE,
        Wrap,
        dispatch::ParserDispatcher,
    },
    iocore::Path,
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let info = Terminal::info();
        let yaml = serde_yaml::to_string(&info)?;
        if info.is_valid {
            println!("{yaml}");
        } else {
            eprintln!("{yaml}");
            std::process::exit(1);
        }

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
