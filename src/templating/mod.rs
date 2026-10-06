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
        Rule::rgb_color => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::color => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::file => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::statement => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::reset => Ok(reduce_pair(&mut pair)?),
        Rule::inner => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::node => Ok(reduce_pairs(&mut pair.into_inner())?),
        Rule::rgb_hex => Ok(reduce_pairs(&mut pair.into_inner())?),
        _ => Ok(format!("not implemented: {}", dbg!(&mut pair))),
        // _ => Ok(pair.as_span().as_str().to_string())
    }
}

pub fn reduce_pair<'a>(pair: &'a mut Pair<Rule>) -> Result<String> {
    let result = pair.as_span().as_str().to_string();
    dbg!(&result, format!("{:#?}", pair.as_rule()));
    Ok(result)
}

pub fn reduce_pairs<'a>(pairs: &'a mut Pairs<Rule>) -> Result<String> {
    // dbg!(&pairs);
    let mut pair = pairs.next().unwrap();
    let result = match pair.as_rule() {
        Rule::file => reduce_pairs(&mut pair.clone().into_inner())?,
        Rule::statement => reduce_pairs(&mut pair.clone().into_inner())?,
        Rule::node => reduce_pairs(&mut pair.clone().into_inner())?,
        Rule::inner => reduce_pairs(&mut pair.clone().into_inner())?,
        Rule::rgb_hex => reduce_pairs(&mut pair.clone().into_inner())?,
        _ => reduce_pair(&mut pair)?,
        // _ => Ok(pair.as_span().as_str().to_string())
    };
    dbg!(&result, format!("{:#?}", pair.as_rule()));

    Ok(result)
}
