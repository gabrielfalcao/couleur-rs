use crate::{Error, Layer, Result, Rule};
use pest::iterators::Pair;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    Color(Color),
    Contrast(crate::contrast::Contrast),
    Layer(crate::layer::Layer),
    Reset(crate::reset::Reset),
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
