use clap::Parser;
use couleur_rs::{Error, Exit, Node, RenderableColor, Result, dispatch::ParserDispatcher, parse};
use winnow::{Parser as _, error::ContextError};
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
}

impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
    pub fn nodes(&self) -> Result<Vec<Node>> {
        let result = parse::<String, ContextError>(self.text())?;
        Ok(result
            .to_vec()
            .into_iter()
            .map(|node| {
                if let Node::Color(color) = &node {
                    Node::RenderableColor(RenderableColor::from(color))
                } else {
                    node
                }
            })
            .collect::<Vec<Node>>())
    }
    pub fn parsed(&self) -> Result<Node> {
        let result = parse::<String, ContextError>(self.text())?;
        Ok(result)
    }
    pub fn rendered(&self) -> Result<String> {
        let parsed = self.parsed()?;
        Ok(parsed.render())
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let input = self.text();
        println!("input: {input}");
        // let parsed = self.parsed()?;
        // println!("parsed: {parsed:#?}");
        let result = self.rendered()?;
        println!("rendered: \x1b[{result}");

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
