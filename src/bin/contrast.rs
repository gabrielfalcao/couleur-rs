#![allow(unused)]
use clap::Parser;
use couleur_rs::{BLACK, Color, Contrast, Error, Exit, Layer, Prefix, Reset, Result, TERMINAL, WHITE, Wrap, dispatch::ParserDispatcher};
use iocore::Path;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "command-line tool to explore the variants of contrast against a color")]
pub struct Cli {
    /// which contrast to use in the background (or foreground if `--invert-layer` is active)
    #[arg()]
    contrast: Contrast,

    /// an RGB color to use in the foreground (or background if `--invert-layer` is active)
    #[arg()]
    color: Color,

    #[arg(short, long, help = "prints the color in the background and contrast in foreground")]
    invert_layer: bool,

    /// text used in the output, optionally provide your own or else
    /// the command defaults to "Hello World"
    #[arg(default_value = "Hello World")]
    text: Vec<String>,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        /// WIP
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
