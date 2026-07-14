// use std::fmt::Display;
// use crate::RGBColor;
// use clap::{ValueEnum, builder::PossibleValue};

#[derive(Clone, Copy, Debug)]
pub enum Layer {
    FG,
    BG,
}
impl Layer {
    pub fn inverted(&self) -> Layer {
        match self {
            Layer::BG => Layer::FG,
            Layer::FG => Layer::BG,
        }
    }
    pub fn code(&self) -> i32 {
        match self {
            Layer::BG => 48,
            Layer::FG => 38,
        }
    }
}
