
use crate::{ Node, RenderableColor, Result, parse};
use winnow::error::ContextError;

pub enum AnsiSequence {
    RenderableColor(RenderableColor),
    Reset,
}
impl AnsiSequence {
    pub fn parse<T: std::fmt::Display>(input: T) -> Result<Node> {
        let result = parse::<String, ContextError>(input.to_string())?;
        Ok(result)
    }
    pub fn nodes<T: std::fmt::Display>(input: T) -> Result<Vec<Node>> {
        let result = Self::parse(input)?;
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
