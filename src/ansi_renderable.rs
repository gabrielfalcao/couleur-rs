use std::fmt::{Debug, Display};

pub trait AnsiRenderable: Sized + Clone + Debug + Display {
    fn render(&self) -> String;
}
