pub use core::colors::Color;
use core::colors::Colorado;
pub use core::style::Style;

pub mod colors;
pub mod symbols;
pub mod style;
pub mod color_string;
pub mod rgb;
pub mod hsl;

pub trait StrMarker {
    fn to_str(&self) -> String;
    fn get_fg_color(&self) -> Option<Colorado> { None }
    fn get_bg_color(&self) -> Option<Colorado> { None }
    fn get_style(&self) -> Option<Vec<Style>> { None }
}

impl<'a> StrMarker for &'a str {
    fn to_str(&self) -> String {
        String::from(*self)
    }
}

