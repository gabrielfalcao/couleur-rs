#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    Algorithm, Colorizer, Error, Exit, Layer, Color, Reset, Result, Wrap,
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
    contrast: Option<Algorithm>,
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
        let text = self.text.join(" ");
        let bg = self.bg.clone();
        let bold = self.bold;
        let fg = self.fg.clone();
        let contrast = self.contrast.unwrap_or_else(|| Algorithm::None);
        let reset = self.reset.unwrap_or_default();
        let wrap = self.wrap.unwrap_or_default();
        let colorizer = Colorizer {
            bg,
            fg,
            contrast,
            wrap,
            bold,
            reset,
        };

        let result = colorizer.colorize(&text)?;
        println!("{text}");
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
