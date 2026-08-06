use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::Color;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct Palette<'a> {
    pub name: &'a str,
    pub colors: HashSet<PaletteColor<'a>>,
}
impl<'a> PaletteColor<'a> {
    pub fn new(name: &'a str, color: Color) -> PaletteColor<'a> {
        Palette { name, color }
    }
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash, Serialize, Deserialize)]
pub struct PaletteColor<'a> {
    name: Option<&'a str>,
    color: Color,
}

impl<'a> PaletteColor<'a> {
    pub fn new(color: Color) -> PaletteColor<'a> {
        PaletteColor { name: None, color }
    }
    pub fn named(name: &'a str, color: Color) -> PaletteColor<'a> {
        PaletteColor { name: Some(name), color }
    }
}

impl<'a> std::ops::Deref for PaletteColor<'a> {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.color
    }
}
impl<'a> From<Color> for PaletteColor<'a> {
    fn from(color: Color) -> PaletteColor<'a> {
        PaletteColor::new(color)
    }
}
impl<'a> Into<Color> for PaletteColor<'a> {
    fn into(self) -> Color {
        self.color
    }
}
