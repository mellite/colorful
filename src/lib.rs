//! Colored your terminal.
//! You can use this package to make your string colorful in terminal.

/// It is recommended to use `Color` enum item to set foreground color.
/// # Examples
/// ```
/// extern crate colorful;
///
/// use colorful::Colorful;
/// use colorful::Color;
///
/// fn main(){
///     let s = "Hello world";
///     println!("{}",s.color(Color::Red).bg_color(Color::Yellow).bold().to_string());
/// }
/// ```
///


use core::color_string::ColorfulString;
pub use core::colors::Color;
pub use core::hsl::HSL;
pub use core::rgb::RGB;
use core::StrMarker;
pub use core::style::Style;

pub mod core;


pub trait Colorful {
    fn color(self, color: Color) -> ColorfulString;
    fn bg_color(self, color: Color) -> ColorfulString;
    fn rgb(self, r: u8, g: u8, b: u8) -> ColorfulString;
    fn bg_rgb(self, r: u8, g: u8, b: u8) -> ColorfulString;
    fn hsl(self, h: f32, s: f32, l: f32) -> ColorfulString;
    fn bg_hsl(self, h: f32, s: f32, l: f32) -> ColorfulString;
    fn style(self, style: Style) -> ColorfulString;
    // style
    fn bold(self) -> ColorfulString;
    fn blink(self) -> ColorfulString;
    fn dim(self) -> ColorfulString;
    fn underlined(self) -> ColorfulString;
    fn reverse(self) -> ColorfulString;
    fn hidden(self) -> ColorfulString;
    // basic usage support 15 color
    // foreground color
    fn black(self) -> ColorfulString;
    fn red(self) -> ColorfulString;
    fn green(self) -> ColorfulString;
    fn yellow(self) -> ColorfulString;
    fn blue(self) -> ColorfulString;
    fn magenta(self) -> ColorfulString;
    fn cyan(self) -> ColorfulString;
    fn light_gray(self) -> ColorfulString;
    fn dark_gray(self) -> ColorfulString;
    fn light_red(self) -> ColorfulString;
    fn light_green(self) -> ColorfulString;
    fn light_yellow(self) -> ColorfulString;
    fn light_blue(self) -> ColorfulString;
    fn light_magenta(self) -> ColorfulString;
    fn light_cyan(self) -> ColorfulString;
    fn white(self) -> ColorfulString;
    // background color
    fn bg_black(self) -> ColorfulString;
    fn bg_red(self) -> ColorfulString;
}

impl<T> Colorful for T where T: StrMarker {
    /// Using enum item is recommended. color will replace
    fn color(self, color: Color) -> ColorfulString {
        ColorfulString::create_by_fg(self, color)
    }
    fn bg_color(self, color: Color) -> ColorfulString {
        ColorfulString::create_by_bg(self, color)
    }
    fn rgb(self, r: u8, g: u8, b: u8) -> ColorfulString {
        ColorfulString::create_by_fg(self, RGB::new(r, g, b))
    }
    fn bg_rgb(self, r: u8, g: u8, b: u8) -> ColorfulString {
        ColorfulString::create_by_bg(self, RGB::new(r, g, b))
    }
    fn hsl(self, h: f32, s: f32, l: f32) -> ColorfulString {
        ColorfulString::create_by_fg(self, HSL::new(h, s, l))
    }
    fn bg_hsl(self, h: f32, s: f32, l: f32) -> ColorfulString {
        ColorfulString::create_by_bg(self, HSL::new(h, s, l))
    }
    fn style(self, style: Style) -> ColorfulString {
        ColorfulString {
            text: String::from(self.to_str()),
            styles: match self.get_style() {
                Some(mut v) => {
                    v.push(style);
                    Some(v)
                }
                _ => { Some(vec![style]) }
            },
            fg_color: self.get_fg_color(),
            bg_color: self.get_bg_color(),
        }
    }
    // style
    fn bold(self) -> ColorfulString { self.style(Style::Bold) }
    fn blink(self) -> ColorfulString { self.style(Style::Blink) }
    fn dim(self) -> ColorfulString { self.style(Style::Dim) }
    fn underlined(self) -> ColorfulString { self.style(Style::Underlined) }
    fn reverse(self) -> ColorfulString { self.style(Style::Reverse) }
    fn hidden(self) -> ColorfulString { self.style(Style::Hidden) }
    // color
    fn black(self) -> ColorfulString { self.color(Color::Black) }
    fn red(self) -> ColorfulString { self.color(Color::Red) }
    fn green(self) -> ColorfulString { self.color(Color::Green) }
    fn yellow(self) -> ColorfulString { self.color(Color::Yellow) }
    fn blue(self) -> ColorfulString { self.color(Color::Blue) }
    fn magenta(self) -> ColorfulString { self.color(Color::Magenta) }
    fn cyan(self) -> ColorfulString { self.color(Color::Cyan) }
    fn light_gray(self) -> ColorfulString { self.color(Color::LightGray) }
    fn dark_gray(self) -> ColorfulString { self.color(Color::DarkGray) }
    fn light_red(self) -> ColorfulString { self.color(Color::LightRed) }
    fn light_green(self) -> ColorfulString { self.color(Color::LightGreen) }
    fn light_yellow(self) -> ColorfulString { self.color(Color::LightYellow) }
    fn light_blue(self) -> ColorfulString { self.color(Color::LightBlue) }
    fn light_magenta(self) -> ColorfulString { self.color(Color::LightMagenta) }
    fn light_cyan(self) -> ColorfulString { self.color(Color::LightCyan) }
    fn white(self) -> ColorfulString { self.color(Color::White) }
    // background color
    fn bg_black(self) -> ColorfulString { self.bg_color(Color::White) }
    fn bg_red(self) -> ColorfulString { self.bg_color(Color::White) }
}


pub trait ExtraColorInterface {
    fn grey0(self) -> ColorfulString;
}

impl<T> ExtraColorInterface for T where T: Colorful {
    fn grey0(self) -> ColorfulString { self.color(Color::Grey0) }
}

