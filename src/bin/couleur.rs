use clap::Parser;
use couleur_rs::{
    Algorithm, Error, Exit, Layer, RGBColor, Reset, Result, Wrap, dispatch::ParserDispatcher,
};
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

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let text = self.text.join(" ");
        let fg = self
            .fg
            .unwrap_or_else(|| "000000".parse().unwrap())
            .wrap_ansi(
                &format!("{text}"),
                Some(Layer::BG),
                true,
                self.wrap,
                self.reset,
                self.contrast,
            );
        println!("{fg}");
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
