#![allow(unused)]
pub mod errors;
pub use errors::{Error, Result};

pub mod contrast;
pub use contrast::Contrast;

pub mod layer;
pub use layer::Layer;

pub mod reset;
pub use reset::Reset;

pub mod ast;
pub use ast::{Color, InvalidMarkupToken, Node, PaletteColor};

#[derive(Parser, Debug, Clone)]
#[grammar = "src/grammar.pest"]
pub struct Definition;
use pest::Parser;
use pest_derive::Parser;

pub fn parse_tokens(input: &str) -> Result<Vec<Node>> {
    let mut pairs = match Definition::parse(Rule::text, input).map_err(|e| Error::ParseError(e.to_string())) {
        Ok(pairs) => pairs,
        Err(e) => {
            log::warn!("{e}");
            return Err(Error::ParseError(e.to_string()));
        }
    };
    let text = pairs.next().unwrap();
    let tokens = Node::from_pair(text)?;
    Ok(tokens)
}
