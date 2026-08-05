#![allow(unused)]
use clap::Parser;
use couleur_rs::{
    BLACK,
    Color,
    Contrast,
    Error,
    Exit,
    Layer,
    Prefix,
    Reset,
    Result,
    TERMINAL,
    WHITE,
    Wrap,
    dispatch::ParserDispatcher,
};
use iocore::Path;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "command-line tool to explore the variants of contrast against a color"
)]
pub struct Cli {
    /// which contrast to use in the background (or foreground if `--invert-layer` is active)
    #[arg()]
    contrast: Contrast,

    /// an RGB color to use in the foreground (or background if `--invert-layer` is active)
    #[arg()]
    color: Color,

    #[arg(short, long, help = "prints the color in the background and contrast in foreground")]
    invert_layer: bool,

    /// text used in the output, optionally provide your own or else
    /// the command defaults to "Hello World"
    #[arg(default_value = "Hello World")]
    text: Vec<String>,
}

impl Cli {}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        let color = self.color;
        let contrast = self.contrast;
        let (color_layer, contrast_layer) =
            if !self.invert_layer { (Layer::FG, Layer::BG) } else { (Layer::BG, Layer::FG) };
        let contrast_color = contrast.apply(color, contrast_layer);
        let reset = Reset::to_ansi(None);

        // first let's ansi-render the color and contrast color in the
        // layers defined by command-line flags
        let color_ansi_normal = color.to_ansi(color_layer);
        let contrast_ansi_normal = contrast_color.to_ansi(contrast_layer);

        // now let's invert the color layers so we can make the output
        // occasionaly contrast with itself
        let color_ansi_inverted = color.to_ansi(contrast_layer);
        let contrast_ansi_inverted = contrast_color.to_ansi(color_layer);

        let input_text = self.text.join(" ");
        let text_lines = vec![
            format!("{reset}color: {color_ansi_normal}{color}{reset}"),
            format!("{reset}contrast: {contrast_ansi_inverted}{contrast_color}{reset}"),
            String::new(),
            format!("{color_ansi_normal}{contrast_ansi_normal}{input_text}{reset}"),
            format!("{color_ansi_inverted}{contrast_ansi_inverted}{input_text}{reset}"),
        ];

        let output_text = text_lines.join("\n");
        println!("{output_text}");
        Ok(())
    }
}

fn main() -> Exit {
    Cli::main()
}
