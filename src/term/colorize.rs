#[derive(Debug, Clone)]
pub struct Input {
    #[arg(long)]
    bg: Option<RGBColor>,
    #[arg(long)]
    fg: Option<RGBColor>,
    #[arg(long)]
    contrast: Option<Algorithm>,
    #[arg(short, long)]
    reset: Option<Reset>,
    #[arg(short, long)]
    wrap: Option<Wrap>,
    #[arg()]
    text: Vec<String>,
}
