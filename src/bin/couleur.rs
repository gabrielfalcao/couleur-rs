#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    Contrast, AnsiColorizer, Color, Error, Exit, Layer, Reset, Result, Wrap,
    dispatch::ParserDispatcher,
};
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {
    #[arg(long, required_unless_present_any = ["fg", "contrast"])]
    bg: Option<Color>,
    #[arg(long, required_unless_present_any = ["bg", "contrast"])]
    fg: Option<Color>,
    #[arg(short, long)]
    bold: bool,
    #[arg(long, required_unless_present_all = ["bg", "fg"])]
    contrast: Option<Contrast>,
    #[arg(short, long)]
    reset: Option<Reset>,
    #[arg(short, short, long)]
    wrap: Option<Wrap>,
    #[arg(short, long, help="detect terminal default colors for background and foreground instead of using contrast", required_unless_present_any=["contrast"])]
    detect: bool,
    #[arg()]
    text: Vec<String>,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let bg = self.bg;
        let bold = self.bold;
        let fg = self.fg;
        let contrast = self.contrast.unwrap_or_default();
        let reset = self.reset.unwrap_or_default();
        let wrap = self.wrap.unwrap_or_default();
        let colorizer = AnsiColorizer {
            bg,
            fg,
            contrast,
            wrap,
            bold,
            reset,
        };
        let result = colorizer.colorize(self.text.join(" "))?;
        println!("{result}");

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
