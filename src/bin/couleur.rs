use clap::Parser;
use couleur_rs::{
    AnsiRenderable,
    Error,
    Exit,
    Node,
    RenderableColor,
    Result,
    ToAnsiEscSuffix,
    dispatch::ParserDispatcher,
    parse,
};
use debug_et_diagnostics::dbg;
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
}
pub fn fold_nodes<T: Iterator<Item = Node>>(nodes: T) -> Vec<Node> {
    nodes.fold(Vec::new(), |mut vec, node| {
        dbg!(&vec, &node);

        match node.clone() {
            Node::Reset(_reset) => {
                vec.push(node.clone());
                vec
            }
            Node::Color(color) => {
                vec.push(Node::RenderableColor(RenderableColor::new(color)));
                vec
            }
            Node::Layer(_layer) => {
                vec.push(node.clone());
                vec
            }
            Node::Contrast(_contrast) => {
                vec.push(node.clone());
                vec
            }
            Node::Text(_string) => {
                vec.push(node.clone());
                vec
            }
            Node::RenderableColor(_renderable_color) => {
                vec.push(node.clone());
                vec
            }
            Node::Array(nodes) => {
                vec.extend(fold_nodes(nodes.into_iter()));
                vec
            }
            Node::EOI => vec,
        }
    })
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
    pub fn parsed(&self) -> Result<Node> {
        let result = parse::<String, ContextError>(self.text())?;
        Ok(result)
    }
    pub fn rendered(&self) -> Result<String> {
        let parsed = self.parsed()?;
        // let mut result = Vec::<String>::new();
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
        println!("rendered: {result}");

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
