#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    BLACK, Color, Contrast, Error, Exit, Layer, Prefix, Reset, Result, TERMINAL, WHITE, Wrap,
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
                let fg = color.to_ansi(Layer::FG);
                let fg_contrast = if color.is_dark() && TERMINAL.is_dark {
                    *WHITE
                } else {
                    *BLACK
                }
                .to_ansi(Layer::BG);

                let bg = color.to_ansi(Layer::BG);
                let bg_contrast = if color.is_dark() && TERMINAL.is_dark {
                    TERMINAL.foreground
                } else {
                    TERMINAL.background
                }
                .to_ansi(Layer::FG);

                let [r, g, b] = color.to_triple();
                println!("{fg_contrast}{fg}{color}\x1b[0m{bg_contrast}{bg}{color}\x1b[0m");
            }
        }

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
