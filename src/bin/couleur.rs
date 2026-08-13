use clap::Parser;
use couleur_rs::{
    AnsiRenderable,
    Error,
    Exit,
    Node,
    RenderableColor,
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
    #[arg(default_value = "{color:#E83B3B}Hello{color:#E83B3B%contrast:web} World")]
    text: Vec<String>,
}
impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
    pub fn nodes(&self) -> Result<Vec<Node>> {
        let result = parse::<String, ContextError>(self.text())?;
        Ok(fold_nodes(result.to_vec().into_iter().map(|node| {
            if let Node::Color(color) = &node {
                Node::RenderableColor(RenderableColor::from(color))
            } else {
                node
            }
        })))
    }
    pub fn rendered(&self) -> Result<String> {
        let nodes = self.nodes()?;
        let result = nodes.into_iter().map(|node| node.render()).collect::<String>();
        Ok(result)
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let input = self.text();
        println!("input: {input}");
        // let parsed = self.parsed()?;
        // println!("parsed: {parsed:#?}");
        let result = self.rendered()?;
        println!("rendered: {result}");

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
