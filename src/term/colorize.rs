use couleur_rs::{
    Algorithm, Error, Exit, Layer, RESET, RGBColor, Reset, Result, Wrap, dispatch::ParserDispatcher,
};

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    bg: Option<RGBColor>,
    fg: Option<RGBColor>,
    contrast: Option<Algorithm>,
    reset: Option<Reset>,
    wrap: Option<Wrap>,
    text: Vec<String>,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let text = self.text.join(" ");
        let layer = if self.bg.is_none() {
            Layer::BG
        } else {
            Layer::FG
        };
        let bg = match self.bg {
            Some(bg) => bg,
            None => RGBColor::default_for_bg()?,
        };
        let fg = match self.fg {
            Some(fg) => fg,
            None => RGBColor::default_for_fg()?,
        };

        let fore = fg.to_ansi(layer, true);
        let back = bg.to_ansi(layer.inverted(), true);
        println!("{back}{fore}{text}{RESET}");
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
