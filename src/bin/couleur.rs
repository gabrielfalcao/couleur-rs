use clap::Parser;
use couleur_rs::{
    AnsiRenderable,
    Error,
    Exit,
    Node,
    RenderableColor,
    RenderingOptions,
    Reset,
    Result,
    dispatch::ParserDispatcher,
    fold_nodes,
    parse,
};
// use winnow::{Parser as _};
use winnow::error::ContextError;
#[derive(Parser, Debug, Clone)]
#[command(
    author = "Gabriel Falcão <gabrielteratos@gmail.com",
    version,
    about,
    long_about = "parse, manipulate, apply contrast algorithms to RGB colors and print in ANSI code"
)]
pub struct Cli {
    #[arg(required = true)]
    text: Vec<String>,
    #[arg(short, long, help = "escape the ANSI sequences")]
    escape: bool,
    #[clap(flatten)]
    opts: RenderingOptions,
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        self.opts.init();
        let rendered = render(self.text.join(" "), self.opts.prefix, self.escape)?;
        if self.escape {
            println!("{result:#?}");
        } else {
            println!("{result}");
        }
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
