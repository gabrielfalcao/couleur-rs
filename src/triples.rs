pub type U8Triple = (u8, u8, u8);

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct RgbTriple(u8, u8, u8);

impl RgbTriple {
    pub fn red(&self) -> u8 {
        self.0
    }
    pub fn green(&self) -> u8 {
        self.1
    }
    pub fn blue(&self) -> u8 {
        self.2
    }
    pub fn into_triple(self) -> U8Triple {
        (self.red(), self.green(), self.blue())
    }
}
impl From<U8Triple> for RgbTriple {
    fn from(input: U8Triple) -> RgbTriple {
        let (red, green, blue) = input;
        RgbTriple(red, green, blue)
    }
}

impl Into<U8Triple> for RgbTriple {
    fn into(self) -> U8Triple {
        self.into_triple()
    }
}
