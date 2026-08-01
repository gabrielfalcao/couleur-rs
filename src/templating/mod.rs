pub mod ast;

pub use ast::{Node, Variable};

pub mod resolve;

#[derive(Parser, Debug, Clone)]
#[grammar = "src/templating/grammar.pest"]
pub struct Definition;
use pest::Parser;
use pest_derive::Parser;

pub fn parse_tokens(input: &str) -> Result<Vec<Node>> {
    let mut pairs = match Definition::parse(Rule::text, input)
        .map_err(|e| Error::ParseError(e.to_string()))
    {
        Ok(pairs) => pairs,
        Err(e) => {
            eprintln!("warning: {}", &e);
            return Ok(Node::default_vec());
        },
    };
    let text = pairs.next().unwrap();
    let tokens = Node::from_pair(text)?;
    Ok(tokens)
}
