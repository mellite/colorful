use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

pub enum Symbol {
    Mode,
    Semicolon,
    LeftBrackets,
    Esc,
    Reset,
    Simple256Foreground,
    Simple256Background,
    RgbForeground,
    RgbBackground
}

impl Symbol {
    pub fn to_str<'a>(&self) -> &'a str {
        match self {
            Symbol::Mode => "m",
            Symbol::Semicolon => ";",
            Symbol::LeftBrackets => "[",
            Symbol::Esc => "\x1B",
            Symbol::Reset => "\x1B[0m",
            Symbol::Simple256Foreground => "\x1B[38;5;",
            Symbol::Simple256Background => "\x1B[48;5;",
            Symbol::RgbForeground => "\x1B[38;2;",
            Symbol::RgbBackground => "\x1B[48;2;",
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_str())
    }
}