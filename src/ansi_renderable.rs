use crate::Node;
use std::iter::{IntoIterator, Iterator};
pub trait AnsiRenderable: Sized + Clone {
    fn render(&self) -> String;
}

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
impl AnsiRenderable for String {
    fn render(&self) -> String {
        self.to_string()
    }
}

impl AnsiRenderable for &Vec<Node> {
    fn render(&self) -> String {
        self.into_iter().map(|node| node.render()).collect::<String>()
    }
}
