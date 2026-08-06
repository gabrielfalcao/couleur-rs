use crate::{Color, Palette, PaletteColor};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// `RuntimeColors` represents named colors and color palettes
/// available during runtime as well as the default terminal bg color to be used when calling [`Terminal::background_color()`] or [`Terminal::foreground_color()`] return errors
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq)]
pub struct RuntimeColors {
    pub(crate) named_colors: HashMap<String, Color>,
    pub(crate) palettes: HashMap<String, Palette>,
    pub(crate) fallback_bg_color: Color,
    pub(crate) fallback_fg_color: Color,
}
impl Default for RuntimeColors {
    fn default() -> RuntimeColors {
        RuntimeColors {
            named_colors: Default::default(),
            palettes: Default::default(),
            fallback_bg_color: Terminal::background_color().unwrap_or_else(|_| Color::BLACK),
            fallback_fg_color: Terminal::foreground_color().unwrap_or_else(|_| Color::WHITE),
        }
    }
}
impl RuntimeColors {
    pub fn new(
        named_colors: HashMap<String, Color>,
        palettes: HashMap<String, Palette>,
        fallback_bg_color: Color,
        fallback_fg_color: Color,
    ) -> RuntimeColors {
        RuntimeColors { named_colors, palettes, fallback_bg_color, fallback_fg_color }
    }
    pub fn add_palette(&mut self, palette: Palette) {
        self.palettes.insert(palette.name.to_string(), palette);
    }
    pub fn remove_palette(&mut self, palette: &Palette) -> Option<Palette> {
        let name = palette.name.to_string();
        self.palettes.remove(name)
    }
    pub fn remove_palette_by_name(&mut self, name: &String) -> Option<Palette> {
        self.palettes.remove(name)
    }

    pub fn add_named_color(&mut self, name: String, color: Color) {
        self.named_colors.insert(name, color);
    }
    pub fn remove_named_color(&mut self, name: &String) -> Option<(String, Color)> {
        self.named_colors.remove_entry(name)
    }

    pub fn set_fallback_bg_color(&mut self, color: Color) {
        self.fallback_bg_color = color;
    }
    pub fn set_fallback_fg_color(&mut self, color: Color) {
        self.fallback_fg_color = color;
    }
    pub fn fallback_bg_color(&self) -> Color {
        self.fallback_bg_color
    }
    pub fn fallback_fg_color(&self) -> Color {
        self.fallback_fg_color
    }
}
