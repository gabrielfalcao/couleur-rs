pub trait AnsiRenderable: Sized + Clone + Copy + std::fmt::Debug {
    fn render(self) -> String;
}
