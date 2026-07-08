use clap::{Parser, Subcommand};

use couleur_rs::{Error, Exit, Result};

use couleur_rs::dispatch::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = "couleur-rs command-line")]
pub struct CouleurRsCli {
    #[command(subcommand)]
    command: TopLevelCommand,
}
impl CouleurRsCli {
    pub fn command(&self) -> TopLevelCommand {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for CouleurRsCli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum TopLevelCommand {
    Hello(HelloOpt),
}
impl SubcommandDispatcher<Error> for TopLevelCommand {
    fn dispatch(&self) -> Result<()> {
        match self {
            TopLevelCommand::Hello(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
pub struct HelloOpt {
    #[arg()]
    text: Vec<String>,
}
impl HelloOpt {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
}
impl ArgsDispatcher<Error> for HelloOpt {
    fn dispatch(&self) -> Result<()> {
        println!("{}", &self.text());

        Ok(())
    }
}

fn main() -> Exit {
    CouleurRsCli::main()
}
