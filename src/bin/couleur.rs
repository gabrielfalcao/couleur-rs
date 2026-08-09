#![allow(unused)]
use {
    clap::Parser,
    couleur_rs::{
        AnsiColorizer,
        Color,
        Contrast,
        Error,
        Exit,
        Layer,
        Prefix,
        Reset,
        Result,
        Wrap,
        dispatch::ParserDispatcher,
    },
};
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct Cli {
    #[arg(default_value = "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World")]
    text: Vec<String>,
}

impl Cli {}

impl<T> ParserDispatcher<Error> for Cli
where
    T: ParserError<Stream<'i>> + AddContext<Stream<'i>, StrContext> + Display,
{
    fn dispatch(&self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
