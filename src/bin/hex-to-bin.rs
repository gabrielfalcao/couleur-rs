#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    AnsiColorizer, Color, Contrast, Error, Exit, Layer, Reset, Result, Wrap,
    dispatch::ParserDispatcher,
};
use iocore::Path;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {
    #[arg()]
    palette_filenames: Vec<Path>,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        for path in self.palette_filenames.iter() {
            let lines = path.read_lines()?;
            for line in lines {
                let color = line.parse::<Color>()?;
                let fg = color.to_ansi(Layer::FG, true);
                let bg = color.to_ansi(Layer::BG, true);
                let fg_contrast = Contrast::Harmonic.apply(color, Layer::BG)?.to_ansi(Layer::FG, true);
                let bg_contrast = Contrast::Web.apply(color, Layer::FG)?.to_ansi(Layer::BG, true);
                let [r,g,b] = color.to_triple();
                println!("{fg_contrast}{fg}{color}\x1b[0m");
            }
        }

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
