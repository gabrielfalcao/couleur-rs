#![allow(unused)]
use std::str::FromStr;

use pest::{
    Parser,
    iterators::{Pair, Pairs},
};
use pest_derive::Parser;

use crate::{Error, Reset, Result};

#[doc(hidden)] pub mod node;
#[doc(inline)] pub use node::{Node, NodeType, fold_nodes};

pub const GRAMMAR: &'static str = include_str!("./grammar.pest");

#[derive(Parser, Debug, Clone)]
#[grammar = "src/templating/grammar.pest"]
pub struct TemplateSource;

pub fn parse<'a>(input: &'a str) -> Result<String> {
    let mut pairs = TemplateSource::parse(Rule::file, input)
        .map_err(|e| Error::ParseError(e.variant.message().to_string().into()))?;
    let pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::rgb_color => Ok(pair.as_span().as_str().to_string()),
        _ => todo!("{}", pair),
    }
}
