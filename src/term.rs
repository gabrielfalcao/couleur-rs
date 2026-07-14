use crate::{ConversionToU8Error, Error, Result, RGBColor};
use owo_colors::Rgb;

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
    pub fn code(&self) -> f32 {
        match self {
            Layer::BG => 48.0,
            Layer::FG => 38.0,
        }
    }
}

pub enum Wrap {
    Before,
    After,
    Around,
}

pub enum Reset {
    Before,
    After,
    Around,
    None,
}

pub enum Algorithms {
    Read,
    HighBit,
    Harmonic,
    Web,
}
pub trait Algorithm {
    fn apply(&self, color: RGBColor) -> RGBColor;
}
