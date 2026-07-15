use crate::{ANSI256Color, RGBColor};

#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub enum Color {
    Rgb(RGBColor),
    // ANSI256(ANSI256Color),
    // ANSISystem(ANSISystemColor),
}
impl Color {
    //def to_ansi(self, layer: Layer, bold: bool = True) -> str:
    pub fn to_ansi(&self, layer: Layer, bold: bool) -> String {
        match self {
            Rgb(rgb) => {
                let triple = rgb.to_triple();
                let prefix = triple.join(";");
                let parts = if bold {
                    vec!["1".to_string]
                } else {
                    Vec::<String>::new()
                };
                parts.push(layer.code().to_string());
                parts.push("2".to_string());
                parts.push(format!("{prefix}m"));
                format!("\x1b[{}", parts.join(";"))
            }
        }
    }

    pub fn wrap_ansi(
        &self,
        text: &str,
        layer: Option<Layer>,
        bold: bool,
        wrap: Option<Wrap>,
        reset: Option<Reset>,
        algorithm: Option<Algorithm>,
    ) -> String {
        let layer = layer.unwrap_or_default();
        let wrap = wrap.unwrap_or_default();
        let reset = reset.unwrap_or_default();
        let algorithm = algorithm.unwrap_or_default();

        let ansi_sequence = self.to_ansi(layer, bold);
        let contrast = self
            .contrast(algorithm)
            .to_ansi(layer = layer.inverted(), bold = bold);

        let colored = match wrap {
            Wrap::Before => format!("{ansi_sequence}{text}"),
            Wrap::After => format!("{text}{ansi_sequence}"),
            Wrap::Around => format!("{ansi_sequence}{text}{ansi_sequence}"),
        };
        let result = match reset {
            Reset::Before => format!("\x1b[0m{colored}"),
            Reset::After => format!("{colored}\x1b[0m"),
            Reset::Around => format!("\x1b[0m{colored}\x1b[0m"),
            Reset::None => colored,
        };
        return result;
    }
}
