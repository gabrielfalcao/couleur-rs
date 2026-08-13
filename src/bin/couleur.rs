use clap::Parser;
use couleur_rs::{Error, Exit, Node, RenderableColor, Result, dispatch::ParserDispatcher, parse};
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

impl Cli {
    pub fn text(&self) -> String {
        self.text.join(" ")
    }
    pub fn nodes(&self) -> Result<Vec<Node>> {
        let result = parse::<String, ContextError>(self.text())?;
        Ok(result.to_vec().into_iter().fold(Vec::<Node>::new(), |vec, node| {
            let mut vec = vec.clone();
            vec.push(match node {
                Node::Color(color) => Node::RenderableColor(RenderableColor::from(color)),
                Node::RenderableColor(color) => Node::RenderableColor(color),
                Node::Text(text) => Node::Text(text),
                other => {
                    other
                    // if vec.len() > 0 {
                    //     match vec[vec.len()-1] {
                    //         Node::RenderableColor(renderable) => {
                    //
                    //         }
                    //     }
                    // }
                }
            });
            vec
        }))
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
