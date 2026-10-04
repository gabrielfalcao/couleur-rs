use clap::Parser;
use couleur_rs::{
    AnsiRenderable,
    Error,
    Exit,
    RenderableColor,
    RenderingOptions,
    Reset,
    Result,
    dispatch::ParserDispatcher,
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
impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
    pub fn rendered(&self) -> Result<String> {
        let result = parse(&self.text())?;

        Ok(result)
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        self.opts.init();
        let result = self.rendered()?;

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
