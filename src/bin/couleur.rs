use clap::Parser;

use couleur_rs::{Error, Exit, Result};
use couleur_rs::{Algorithm, Reset, Wrap, Layer, RGBColor};
use couleur_rs::dispatch::ParserDispatcher;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {
    #[arg(long)]
    bg: Option<RGBColor>,

    #[arg(long)]
    fg: Option<RGBColor>,

    #[arg(long)]
    contrast: Option<Algorithm>,

    #[arg(short, long)]
    reset: Option<Reset>,

    #[arg(short, long)]
    wrap: Option<Wrap>,

    #[arg()]
    text: Vec<String>,
}
impl Cli {

}
impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        println!("{}", self.text.join(" "));
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
