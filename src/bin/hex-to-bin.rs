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
                let fg_colorizer = AnsiColorizer {
                    bg: None,
                    fg: Some(color),
                    contrast: Contrast::Harmonic,
                    wrap: Wrap::default(),
                    bold: true,
                    reset: Reset::default(),
                };
                let bg_colorizer = AnsiColorizer {
                    bg: Some(color),
                    fg: None,
                    contrast: Contrast::Web,
                    wrap: Wrap::default(),
                    bold: true,
                    reset: Reset::default(),
                };
                let [r,g,b] = color.to_triple();
                let fg = fg_colorizer.colorize(&format!("#{line}"))?;
                let bg = bg_colorizer.colorize(&format!("{r}, {g}, {b}"))?;
                println!("{fg}\t{bg}");
            }
        }

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
