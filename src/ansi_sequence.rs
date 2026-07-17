use couleur_rs::{
    Algorithm, Error, Exit, Layer, RESET, Color, Reset, Result, Wrap, dispatch::ParserDispatcher,
};

#[derive(Parser, Debug, Clone, Copy)]
pub struct AnsiSequence {
    bg: Option<Color>,
    fg: Option<Color>,
    contrast: Option<Algorithm>,
    reset: Option<Reset>,
    wrap: Option<Wrap>,
}
impl AnsiSequence {
    pub fn wrap(mut self, wrap: Wrap) -> AnsiSequence {
        self.wrap = Some(wrap);
        self
    }
    pub fn reset(mut self, reset: Reset) -> AnsiSequence {
        self.reset = Some(reset);
        self
    }
    pub fn contrast(mut self, contrast: Algorithm) -> AnsiSequence {
        self.contrast = Some(contrast);
        self
    }
}

#[cfg(test)]
mod test {
    use crate::{Algorithm, Layer, Color, Value, Reset, Result, Wrap};
    use k9::assert_equal;

    #[test]
    fn test_fg_contrast() -> Result<()> {
        let fore = AnsiSequenceBuilder::bg("FFCC00".parse::<Color>().unwrap())
            .wrap(Wrap::Before)
            .reset(Reset::After)
            .contrast(Algorithm::Harmonic);
    }
}
