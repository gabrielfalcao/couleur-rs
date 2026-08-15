#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    AnsiRenderable,
    Color,
    Contrast,
    Error,
    Exit,
    Layer,
    RenderableColor,
    RenderingOptions,
    Reset,
    Result,
    dispatch::ParserDispatcher,
    get_log_path,
    local_data_dir,
};
use iocore::Path;
#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "temporary tool to display useful information from inside the crate"
)]
pub struct Cli {}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        println!("local_data_dir: {path}", path = local_data_dir());
        println!("log_path: {path}", path = get_log_path()?);
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
