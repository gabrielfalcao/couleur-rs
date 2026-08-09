pub trait AnsiRenderable: Sized + Clone {
    fn render(&self) -> String;
}
