#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB {
    // range 0 -255
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
    pub fn unpack(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
}

impl ToString for RGB {
    fn to_string(&self) -> String {
        format!("{};{};{}", self.r, self.g, self.b)
    }
}
