use enum_to_string::EnumToString;

#[derive(Clone, Copy, EnumToString)]
pub enum ANSISystemColor {
    Black,         // (fg: 30) (bg: 40)
    Red,           // (fg: 31) (bg: 41)
    Green,         // (fg: 32) (bg: 42)
    Yellow,        // (fg: 33) (bg: 43)
    Blue,          // (fg: 34) (bg: 44)
    Magenta,       // (fg: 35) (bg: 45)
    Cyan,          // (fg: 36) (bg: 46)
    White,         // (fg: 37) (bg: 47)
    Gray,          // (fg: 90) (bg: 100)
    Grey,          // (fg: 90) (bg: 100)
    BrightBlack,   // (fg: 90) (bg: 100)
    BrightRed,     // (fg: 91) (bg: 101)
    BrightGreen,   // (fg: 92) (bg: 102)
    BrightYellow,  // (fg: 93) (bg: 103)
    BrightBlue,    // (fg: 94) (bg: 104)
    BrightMagenta, // (fg: 95) (bg: 105)
    BrightCyan,    // (fg: 96) (bg: 106)
    BrightWhite,   // (fg: 97) (bg: 107)
}

impl ANSISystemColor {
    pub fn colors(&self) -> [ANSISystemColor; 16] {
        [
            Black,
            Red,
            Green,
            Yellow,
            Blue,
            Magenta,
            Cyan,
            White,
            Gray,
            Grey,
            BrightBlack,
            BrightRed,
            BrightGreen,
            BrightYellow,
            BrightBlue,
            BrightMagenta,
            BrightCyan,
            BrightWhite,
        ]
    }
    pub fn fg(&self) -> u8 {
        use ANSISystemColor::*;
        match self {
            Black => 30,
            Red => 31,
            Green => 32,
            Yellow => 33,
            Blue => 34,
            Magenta => 35,
            Cyan => 36,
            White => 37,
            Gray => 90,
            Grey => 90,
            BrightBlack => 90,
            BrightRed => 91,
            BrightGreen => 92,
            BrightYellow => 93,
            BrightBlue => 94,
            BrightMagenta => 95,
            BrightCyan => 96,
            BrightWhite => 97,
        }
    }
    pub fn bg(&self) -> u8 {
        use ANSISystemColor::*;
        match self {
            Black => 40,
            Red => 41,
            Green => 42,
            Yellow => 43,
            Blue => 44,
            Magenta => 45,
            Cyan => 46,
            White => 47,
            Gray => 100,
            Grey => 100,
            BrightBlack => 100,
            BrightRed => 101,
            BrightGreen => 102,
            BrightYellow => 103,
            BrightBlue => 104,
            BrightMagenta => 105,
            BrightCyan => 106,
            BrightWhite => 107,
        }
    }
}
