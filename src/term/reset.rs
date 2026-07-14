// use std::fmt::Display;
// use crate::RGBColor;
// use clap::{ValueEnum, builder::PossibleValue};

#[derive(Clone, Copy, Debug)]
pub enum Reset {
    Before,
    After,
    Around,
    None,
}
