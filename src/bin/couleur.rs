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
        let layer = if self.bg.is_none() {
            Layer::BG
        } else {
            Layer::FG
        };
        let bg = match self.bg {
            Some(bg) => bg,
            None => RGBColor::default_for_bg()?,
        };
        let fg = match self.fg {
            Some(fg) => fg,
            None => RGBColor::default_for_fg()?,
        };

        let fore = fg.to_ansi(layer, true);
        let back = bg.to_ansi(layer.inverted(), true);
        println!("{back}{fore}{text}\x1b[0m");
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
