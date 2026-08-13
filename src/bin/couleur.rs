use clap::Parser;
use couleur_rs::{
    AnsiRenderable,
    Error,
    Exit,
    Node,
    RenderableColor,
    Reset,
    Result,
    SharedRenderingOpts,
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

    #[clap(flatten)]
    opts: SharedRenderingOpts,
}
impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
    pub fn nodes(&self) -> Result<Vec<Node>> {
        let result = parse::<String, ContextError>(self.text())?;
        let mut nodes = fold_nodes(result.to_vec().into_iter().map(|node| {
            if let Node::Color(color) = &node {
                Node::RenderableColor(RenderableColor::from(color))
            } else {
                node
            }
        }));
        if nodes.is_empty() {
            return Ok(nodes);
        }
        let last_node_does_not_reset_sequence =
            nodes.last().map(|node| !node.clone().is_reset()).unwrap_or_default();
        if last_node_does_not_reset_sequence && !self.opts.add_reset_to_last_node() {
            nodes.push(Node::Reset(Reset::new(self.opts.prefix())));
        }

        Ok(nodes)
    }
    pub fn rendered(&self) -> Result<String> {
        let nodes = self.nodes()?;
        let result = nodes.into_iter().map(|node| node.render()).collect::<String>();
        Ok(result)
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        self.opts.init();
        let input = self.text();
        println!("input: {input}");
        // let parsed = self.parsed()?;
        // println!("parsed: {parsed:#?}");
        let result = self.rendered()?;
        println!("rendered: {result:#?}");
        println!("rendered: {result}");

        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
