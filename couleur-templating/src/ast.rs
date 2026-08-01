use crate::{Error, Layer, Result, Rule};
use pest::iterators::{Pair, Pairs};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Color(Color),
    Contrast(crate::contrast::Contrast),
    Layer(crate::layer::Layer),
    Reset(crate::reset::Reset),
    Markup(Markup),
    Unhandled(String),
    UnhandledRule(String, String),
    InvalidMarkup(InvalidMarkupToken),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PaletteColor {
    pub palette_name: String,
    pub color_name: String,
}
impl std::fmt::Display for PaletteColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{palette}: {color}", palette = &self.palette_name, color = &self.color_name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    FromPalette(PaletteColor), // (palette_name: String, color_name: String)
    Named(String),
    Terminal(Layer),
    Rgb(String),
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl Color {
    pub fn to_str(&self) -> String {
        match self {
            Color::FromPalette(value) => {
                value.to_string() // PaletteColor
            }
            Color::Named(value) => {
                value.to_string() // String
            }
            Color::Terminal(value) => {
                value.to_string() // Layer
            }
            Color::Rgb(value) => {
                value.to_string() // crate::color::Color
            }
        }
    }

    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Color> {
        match pair.as_rule() {
            Rule::color_rgb => Ok(Color::Rgb(pair.as_span().as_str().to_string())),
            rule => unreachable!("rule {rule:#?} should not reach this code"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MarkupContent {
    Color(Color),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Markup {
    Color(Color),
    Content(Vec<Markup>),
    Unhandled(String),
}
impl Markup {
    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Vec<Markup>> {
        let mut result = Vec::new();
        match pair.as_rule() {
            Rule::color => {
                for pair in pair.clone().into_inner() {
                    result.push(Markup::Color(Color::from_pair(pair)?));
                }
            }
            Rule::rgb_hex => {
                result.push(Markup::Color(Color::from_pair(pair)?));
            },
            rule => unreachable!("rule {rule:#?} should not reach this code"),
        }
        Ok(result)
    }
    pub fn from_pairs<'a>(pairs: Pairs<'a, Rule>) -> Result<Vec<Markup>> {
        let mut result = Vec::<Markup>::new();
        for pair in pairs {
            result.extend(Markup::from_pair(pair)?);
        }
        Ok(result)
    }
}
impl Node {
    fn u8_from_pair<'a>(pair: Pair<'a, Rule>) -> Result<u8> {
        Ok(u8::from_str_radix(pair.as_span().as_str(), 10)
            .map_err(|e| Error::ParseError(format!("{} (expected number from 0 to 255: {:#?})", e, pair.clone())))?)
    }

    pub fn from_pair<'a>(pair: Pair<'a, Rule>) -> Result<Vec<Node>> {
        let mut tokens = Vec::<Node>::new();
        tokens.extend(match pair.as_rule() {
            Rule::node | Rule::text | Rule::inner | Rule::color_rgb_hex => {
                let mut tokens = Vec::<Node>::new();
                for node in pair.clone().into_inner() {
                    tokens.extend(Node::from_pair(node)?);
                }
                tokens
            }
            Rule::markup => {
                Markup::from_pairs(pair.clone().into_inner())?.into_iter().map(|markup| Node::Markup(markup)).collect::<Vec<Node>>()
            }
            Rule::unhandled => {
                vec![Node::Unhandled(pair.as_span().as_str().to_string())]
            }
            Rule::EOI | Rule::WHITESPACE => Vec::<Node>::new(),
            unknown => {
                vec![Node::UnhandledRule(format!("{unknown:#?}"), pair.as_span().as_str().to_string())]
            }
        });
        Ok(tokens)
    }

    pub fn to_str(&self) -> String {
        String::new()
    }
}
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
// impl std::fmt::Debug for Node {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{:#?}", self.to_str())
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InvalidMarkupToken {
    InvalidMarkupAllButClose(String), // @{ (!("}"))+ }
    InvalidMarkupDoubleClose(String), // @{ (!("}"))+ ~ (("}") ~ (!("}"))* }
    InvalidMarkupDoubleOpen(String),  // @{ (!("}"))+ ~ (("{") ~ (!("}"))* }
    InvalidMarkupAny(String),         // @{ ANY+ }
}
