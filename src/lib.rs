//! Colored your terminal.
//! You can use this package to make your string colorful in terminal.

use std::borrow::Cow;
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

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Add;

pub use core::colors::Color;
use core::colors::Colorado;
use core::colors::ColorMode;
pub use core::colors::HSL;
pub use core::colors::hsl_to_rgb;
pub use core::colors::RGB;
pub use core::style::Style;
use core::symbols::Symbol;

pub mod core;

// Support multiple style
pub struct ColorfulString {
    text: String,
    fg_color: Option<Colorado>,
    bg_color: Option<Colorado>,
    styles: Option<Vec<Style>>,
}

impl<'a> Add<&'a str> for ColorfulString {
    type Output = Cow<'a, str>;

    #[inline]
    fn add(self, rhs: &'a str) -> Self::Output {
        let mut s = self.to_string();
        s.push_str(rhs);
        Cow::Owned(s)
    }
}

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

impl StrMarker for ColorfulString {
    fn to_str(&self) -> String {
        self.text.to_owned()
    }
    fn get_fg_color(&self) -> Option<Colorado> {
        self.fg_color.clone()
    }
    fn get_bg_color(&self) -> Option<Colorado> {
        self.bg_color.clone()
    }
    fn get_style(&self) -> Option<Vec<Style>> {
        self.styles.clone()
    }
}

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
        ColorfulString {
            text: String::from(self.to_str()),
            fg_color: Some(Colorado::new(color)),
            bg_color: self.get_bg_color(),
            styles: self.get_style(),
        }
    }
    fn bg_color(self, color: Color) -> ColorfulString {
        ColorfulString {
            text: String::from(self.to_str()),
            fg_color: self.get_fg_color(),
            bg_color: Some(Colorado::new(color)),
            styles: self.get_style(),
        }
    }
    fn rgb(self, r: u8, g: u8, b: u8) -> ColorfulString {
        ColorfulString {
            text: String::from(self.to_str()),
            fg_color: Some(Colorado::new(RGB::new(r, g, b))),
            bg_color: self.get_bg_color(),
            styles: self.get_style(),
        }
    }
    fn bg_rgb(self, r: u8, g: u8, b: u8) -> ColorfulString {
        ColorfulString {
            text: String::from(self.to_str()),
            fg_color: self.get_fg_color(),
            bg_color: Some(Colorado::new(RGB::new(r, g, b))),
            styles: self.get_style(),
        }
    }
    fn hsl(self, h: f32, s: f32, l: f32) -> ColorfulString {
        let c = hsl_to_rgb(HSL::new(h, s, l));
        let (r, g, b) = c.unpack();
        self.rgb(r, g, b)
    }
    fn bg_hsl(self, h: f32, s: f32, l: f32) -> ColorfulString {
        let c = hsl_to_rgb(HSL::new(h, s, l));
        let (r, g, b) = c.unpack();
        self.bg_rgb(r, g, b)
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


impl Display for ColorfulString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut is_colored = false;

        if self.bg_color.is_none() && self.fg_color.is_none() && self.styles.is_none() {
            write!(f, "{}", self.text)?;
            Ok(())
        } else {
            match &self.fg_color {
                Some(v) => {
                    is_colored = true;
                    match v.mode {
                        ColorMode::SIMPLE => {
                            f.write_str(Symbol::Simple256Foreground.to_str())?;
                        }
                        ColorMode::RGB => {
                            f.write_str(Symbol::RgbForeground.to_str())?;
                        }
                        _ => {}
                    }
                    write!(f, "{}", v.color)?;
                }
                _ => {}
            }
            match &self.bg_color {
                Some(v) => {
                    if is_colored {
                        f.write_str(Symbol::Mode.to_str())?;
                    } else {
                        is_colored = true;
                    }
                    match v.mode {
                        ColorMode::SIMPLE => {
                            f.write_str(Symbol::Simple256Background.to_str())?;
                        }
                        ColorMode::RGB => {
                            f.write_str(Symbol::RgbBackground.to_str())?;
                        }
                        _ => {}
                    }
                    write!(f, "{}", v.color)?;
                }
                _ => {}
            }

            match &self.styles {
                Some(v) => {
                    if !is_colored { // pure style without color
                        write!(f, "{}{}", Symbol::Esc, Symbol::LeftBrackets)?;
                    } else {
                        f.write_str(Symbol::Semicolon.to_str())?;
                    }
                    let t: Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                    f.write_str(&t.join(";")[..])?;
                }
                _ => {}
            }
            f.write_str(Symbol::Mode.to_str())?;
            write!(f, "{}", self.text)?;
            f.write_str(Symbol::Reset.to_str())?;
            Ok(())
        }
    }
}