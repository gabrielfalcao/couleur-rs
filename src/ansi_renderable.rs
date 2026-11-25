use std::{
    fmt::{Debug, Display},
    iter::{IntoIterator, Iterator},
};

use crate::{AnsiRenderable, ToAnsiEscSuffix};
use std::{convert::AsRef, ops::Deref};

// impl<T> AnsiRenderable for T
// where
//     T: std::ops::Deref<Target = str> + Clone + Debug + Display,
// {
//     fn render(&self) -> String {
//         self.to_string()
//     }
// }
impl AnsiRenderable for &str {
    fn render(&self) -> String {
        self.to_string()
    }
}
impl ToAnsiEscSuffix for &str {
    fn to_ansi_esc_suffix(&self) -> String {
        self.to_string()
    }
}

impl AnsiRenderable for String {
    fn render(&self) -> String {
        self.to_string()
    }
}
impl ToAnsiEscSuffix for String {
    fn to_ansi_esc_suffix(&self) -> String {
        self.to_string()
    }
}
