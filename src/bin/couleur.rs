#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    Contrast, AnsiColorizer, Color, Error, Exit, Layer, Reset, Result, Wrap,
    dispatch::ParserDispatcher,
};
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {
    #[arg(long)]
    bg: Option<Color>,
    #[arg(long)]
    fg: Option<Color>,
    #[arg(long)]
    bold: bool,
    #[arg(long)]
    contrast: Option<Contrast>,
    #[arg(short, long)]
    reset: Option<Reset>,
    #[arg(short, long)]
    wrap: Option<Wrap>,
    #[arg()]
    text: Vec<String>,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let bg = None;
        let bold = true;
        let fg = Some("#FFCC00".parse::<Color>()?);
        let contrast = Contrast::None;
        let reset = Reset::After;
        let wrap = Wrap::Before;
        let colorizer = AnsiColorizer {
            bg,
            fg,
            contrast,
            wrap,
            bold,
            reset,
        };
        let result = colorizer.colorize("test 123")?;
        println!("{result}");

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
