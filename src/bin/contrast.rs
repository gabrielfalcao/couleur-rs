use clap::Parser;
use couleur_rs::{
    Color,
    Contrast,
    Error,
    Exit,
    Layer,
    RenderingOptions,
    RenderableColor,
    AnsiRenderable,
    Reset,
    Result,
    dispatch::ParserDispatcher,
};

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "displays the given color followed by each contrast algorithm available in the tool"
)]
pub struct Cli {
    /// an RGB color to use in the foreground (or background if `--invert-layer` is active)
    #[arg()]
    color: Color,

    #[clap(flatten)]
    opts: RenderingOptions,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&mut self) -> Result<()> {
        self.opts.init();
        let reset = Reset::new(self.opts.prefix()).render();
        let color = self.color;
        let renderable_color = RenderableColor::new(color);
        // let text = "#".repeat(6);
        let colorize = renderable_color.render();
        println!("{color} => {colorize}{color}{reset}");
        for variant in Contrast::variants() {
            let contrast_color_fg =  variant.apply(color, Layer::FG);
            let renderable_color = RenderableColor::new(contrast_color_fg).with_layer(Layer::FG);
            let colorize_fg = renderable_color.render();

            let contrast_color_bg =  variant.apply(color, Layer::BG);
            let renderable_color = RenderableColor::new(contrast_color_bg).with_layer(Layer::BG);
            let colorize_bg = renderable_color.render();

            let contrast_color_fg_bg =  variant.apply(contrast_color_fg, Layer::BG);
            let renderable_color = RenderableColor::new(contrast_color_fg_bg).with_layer(Layer::BG);
            let colorize_fg_bg = renderable_color.render();

            let contrast_color_bg_fg =  variant.apply(contrast_color_bg, Layer::FG);
            let renderable_color = RenderableColor::new(contrast_color_bg_fg).with_layer(Layer::FG);
            let colorize_bg_fg = renderable_color.render();

            let name = variant.variant_name_snake();
            println!("{contrast_color_fg} => {colorize_fg}{colorize_fg_bg}{contrast_color_fg}{reset} => {colorize_bg}{colorize_bg_fg}{name}{reset}");
        }
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
