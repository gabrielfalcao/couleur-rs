use std::{
    fmt::{Debug, Display},
    iter::{IntoIterator, Iterator},
};

use crate::{Node, Prefix};
pub trait AnsiRenderable: Sized + Clone {
    fn render(&self) -> String {
        self.render_without_prefix()
    }
    // fn to_ansi(&self, prefix: Option<Prefix>) -> String {
    //     format!(
    //         "{prefix}{code}",
    //         prefix = prefix.unwrap_or_default(),
    //         code = self.render_without_prefix()
    //     )
    // }

    fn render_without_prefix(&self) -> String;
}
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
    fn render_without_prefix(&self) -> String {
        self.to_string()
    }
}
impl AnsiRenderable for String {
    fn render_without_prefix(&self) -> String {
        self.to_string()
    }
}

impl AnsiRenderable for &Vec<Node> {
    fn render_without_prefix(&self) -> String {
        self.into_iter().map(|node| node.render()).collect::<String>()
    }
}
