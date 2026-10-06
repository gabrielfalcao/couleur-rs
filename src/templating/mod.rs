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
    let mut pair = pairs.next().unwrap();
    match pair.as_rule() {
        Rule::rgb_color => Ok(reduce_pair(&mut pair)?),
        Rule::color => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::file => Ok(reduce_pair(&mut pair)?),
        Rule::statement => Ok(reduce_pair(&mut pair)?),
        Rule::reset => Ok(reduce_pair(&mut pair)?),
        Rule::inner => Ok(reduce_pair(&mut pair)?),
        Rule::node => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::rgb_hex => Ok(reduce_pairs(&mut pair.into_inner())?),
        _ => Ok(format!("not implemented: {}", dbg!(&mut pair))),
        // _ => Ok(pair.as_span().as_str().to_string())
    }
}

pub fn reduce_pair<'a>(pair: &'a mut Pair<Rule>) -> Result<String> {
    Ok(pair.as_span().as_str().to_string())
}

pub fn reduce_pairs<'a>(pair: &'a mut Pairs<Rule>) -> Result<String> {
    Ok(reduce_pair(&mut pair.next().unwrap())?)
}
