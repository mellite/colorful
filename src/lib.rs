//! Colored your terminal.
//! You can use this package to make your string colorful in terminal.
//! Platform support:
//!  - Linux
//!  - macOS

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

use std::{thread, time};

use core::color_string::CString;
use core::ColorInterface;
pub use core::colors::Color;
pub use core::hsl::HSL;
pub use core::rgb::RGB;
use core::StrMarker;
pub use core::style::Style;

pub mod core;


pub trait Colorful {
    fn color<C: ColorInterface>(self, color: C) -> CString;
    fn bg_color<C: ColorInterface>(self, color: C) -> CString;
    fn rgb(self, r: u8, g: u8, b: u8) -> CString;
    fn bg_rgb(self, r: u8, g: u8, b: u8) -> CString;
    fn hsl(self, h: f32, s: f32, l: f32) -> CString;
    fn bg_hsl(self, h: f32, s: f32, l: f32) -> CString;
    fn style(self, style: Style) -> CString;
    // style
    /// Turn bold mode on.
    fn bold(self) -> CString;
    /// Turn blinking mode on. Blink doesn't work in many terminal emulators ,and it will still work on the console.
    fn blink(self) -> CString;
    /// Turn low intensity mode on.
    fn dim(self) -> CString;
    /// Turn underline mode on.
    fn underlined(self) -> CString;
    /// Turn reverse mode on (invert the foreground and background colors).
    fn reverse(self) -> CString;
    /// Turn invisible text mode on (useful for passwords).
    fn hidden(self) -> CString;
    // Foreground color. Support 16 colors
    fn black(self) -> CString;
    fn red(self) -> CString;
    fn green(self) -> CString;
    fn yellow(self) -> CString;
    fn blue(self) -> CString;
    fn magenta(self) -> CString;
    fn cyan(self) -> CString;
    fn light_gray(self) -> CString;
    fn dark_gray(self) -> CString;
    fn light_red(self) -> CString;
    fn light_green(self) -> CString;
    fn light_yellow(self) -> CString;
    fn light_blue(self) -> CString;
    fn light_magenta(self) -> CString;
    fn light_cyan(self) -> CString;
    fn white(self) -> CString;
    // background color
    fn bg_black(self) -> CString;
    fn bg_red(self) -> CString;
    fn bg_green(self) -> CString;
    fn bg_yellow(self) -> CString;
    fn bg_blue(self) -> CString;
    fn bg_magenta(self) -> CString;
    fn bg_cyan(self) -> CString;
    fn bg_light_gray(self) -> CString;
    fn bg_dark_gray(self) -> CString;
    fn bg_light_red(self) -> CString;
    fn bg_light_green(self) -> CString;
    fn bg_light_yellow(self) -> CString;
    fn bg_light_blue(self) -> CString;
    fn bg_light_magenta(self) -> CString;
    fn bg_light_cyan(self) -> CString;
    fn bg_white(self) -> CString;
    // gradient
    fn gradient<C: ColorInterface>(self, color: C) -> CString;
    fn gradient_with_step<C: ColorInterface>(self, color: C, step: f32) -> CString;
    fn gradient_with_color<C: ColorInterface>(self, start: C, stop: C) -> CString;
    // animations
    fn rainbow_with_speed(self, speed: i32);
    fn rainbow(self);
    fn neon_with_speed<C: ColorInterface>(self, low: C, high: C, speed: i32);
    fn neon<C: ColorInterface>(self, low: C, high: C);
    fn warn(self);
}

impl<T> Colorful for T where T: StrMarker {
    /// Using enum item is recommended. color will replace
    fn color<C: ColorInterface>(self, color: C) -> CString { CString::create_by_fg(self, color) }
    fn bg_color<C: ColorInterface>(self, color: C) -> CString { CString::create_by_bg(self, color) }
    fn rgb(self, r: u8, g: u8, b: u8) -> CString { CString::create_by_fg(self, RGB::new(r, g, b)) }
    fn bg_rgb(self, r: u8, g: u8, b: u8) -> CString { CString::create_by_bg(self, RGB::new(r, g, b)) }
    fn hsl(self, h: f32, s: f32, l: f32) -> CString { CString::create_by_fg(self, HSL::new(h, s, l)) }
    fn bg_hsl(self, h: f32, s: f32, l: f32) -> CString { CString::create_by_bg(self, HSL::new(h, s, l)) }
    fn style(self, style: Style) -> CString { CString::create_by_style(self, style) }
    // style
    fn bold(self) -> CString { self.style(Style::Bold) }
    fn blink(self) -> CString { self.style(Style::Blink) }
    fn dim(self) -> CString { self.style(Style::Dim) }
    fn underlined(self) -> CString { self.style(Style::Underlined) }
    fn reverse(self) -> CString { self.style(Style::Reverse) }
    fn hidden(self) -> CString { self.style(Style::Hidden) }
    // color
    fn black(self) -> CString { self.color(Color::Black) }
    fn red(self) -> CString { self.color(Color::Red) }
    fn green(self) -> CString { self.color(Color::Green) }
    fn yellow(self) -> CString { self.color(Color::Yellow) }
    fn blue(self) -> CString { self.color(Color::Blue) }
    fn magenta(self) -> CString { self.color(Color::Magenta) }
    fn cyan(self) -> CString { self.color(Color::Cyan) }
    fn light_gray(self) -> CString { self.color(Color::LightGray) }
    fn dark_gray(self) -> CString { self.color(Color::DarkGray) }
    fn light_red(self) -> CString { self.color(Color::LightRed) }
    fn light_green(self) -> CString { self.color(Color::LightGreen) }
    fn light_yellow(self) -> CString { self.color(Color::LightYellow) }
    fn light_blue(self) -> CString { self.color(Color::LightBlue) }
    fn light_magenta(self) -> CString { self.color(Color::LightMagenta) }
    fn light_cyan(self) -> CString { self.color(Color::LightCyan) }
    fn white(self) -> CString { self.color(Color::White) }
    // background color
    fn bg_black(self) -> CString { self.bg_color(Color::Black) }
    fn bg_red(self) -> CString { self.bg_color(Color::Red) }
    fn bg_green(self) -> CString { self.bg_color(Color::Green) }
    fn bg_yellow(self) -> CString { self.bg_color(Color::Yellow) }
    fn bg_blue(self) -> CString { self.bg_color(Color::Blue) }
    fn bg_magenta(self) -> CString { self.bg_color(Color::Magenta) }
    fn bg_cyan(self) -> CString { self.bg_color(Color::Cyan) }
    fn bg_light_gray(self) -> CString { self.bg_color(Color::LightGray) }
    fn bg_dark_gray(self) -> CString { self.bg_color(Color::DarkGray) }
    fn bg_light_red(self) -> CString { self.bg_color(Color::LightRed) }
    fn bg_light_green(self) -> CString { self.bg_color(Color::LightGreen) }
    fn bg_light_yellow(self) -> CString { self.bg_color(Color::LightYellow) }
    fn bg_light_blue(self) -> CString { self.bg_color(Color::LightBlue) }
    fn bg_light_magenta(self) -> CString { self.bg_color(Color::LightMagenta) }
    fn bg_light_cyan(self) -> CString { self.bg_color(Color::LightCyan) }
    fn bg_white(self) -> CString { self.bg_color(Color::White) }
    // gradient
    fn gradient<C: ColorInterface>(self, color: C) -> CString {
        self.gradient_with_step(color, 1.5 / 360.0)
    }
    fn gradient_with_step<C: ColorInterface>(self, color: C, step: f32) -> CString {
        let mut t = vec![];
        let mut start = color.to_hsl().h;
        let s = self.to_str();
        let c = s.chars();
        let length = c.clone().count() - 1;
        for (index, i) in c.enumerate() {
            let b = i.to_string();
            let tmp = b.hsl(start, 1.0, 0.5).to_string();
            t.push(format!("{}", &tmp[..tmp.len() - if index != length { 4 } else { 0 }]));
            start = (start + step) % 1.0;
        }
        CString::create_by_text(self, t.join(""))
    }
    fn gradient_with_color<C: ColorInterface>(self, start: C, stop: C) -> CString {
        let mut t = vec![];
        let c = self.to_str();
        let s = c.chars();
        let length = s.clone().count() - 1;
        let mut start = start.to_hsl().h;
        let stop = stop.to_hsl().h;
        let step = (stop - start) / length as f32;
        for (index, i) in s.enumerate() {
            let b = i.to_string();
            let tmp = b.hsl(start, 1.0, 0.5).to_string();
            t.push(format!("{}", &tmp[..tmp.len() - if index != length { 4 } else { 0 }]));
            start = (start + step) % 1.0;
        }
        CString::create_by_text(self, t.join(""))
    }
    fn rainbow_with_speed(self, speed: i32) {
        let respite: u64 = match speed {
            3 => { 10 }
            2 => { 5 }
            1 => { 2 }
            _ => { 0 }
        };
        let text = self.to_str();
        let lines: Vec<_> = text.lines().collect();
        for i in 0..360 {
            let mut tmp = vec![];
            for sub_str in lines.iter() {
                tmp.push(sub_str.gradient_with_step(HSL::new(i as f32 / 360.0, 1.0, 0.5), 0.02).to_string());
            }
            println!("{}\x1B[{}F\x1B[G\x1B[2K", tmp.join("\n"), lines.len());
            let ten_millis = time::Duration::from_millis(respite);
            thread::sleep(ten_millis);
        }
    }
    fn rainbow(self) {
        self.rainbow_with_speed(3);
    }
    fn neon_with_speed<C: ColorInterface>(self, high: C, low: C, speed: i32) {
        let respite: u64 = match speed {
            3 => { 500 }
            2 => { 200 }
            1 => { 100 }
            _ => { 0 }
        };
        let text = self.to_str();
        let lines: Vec<_> = text.lines().collect();
        let mut coin = true;
        let positive = format!("{}\x1B[{}F\x1B[2K", text.color(high), lines.len());
        let negative = format!("{}\x1B[{}F\x1B[2K", text.color(low), lines.len());
        for _ in 0..360 {
            if coin { println!("{}", positive) } else { println!("{}", negative) };
            let ten_millis = time::Duration::from_millis(respite);
            thread::sleep(ten_millis);
            coin = !coin;
        }
    }
    fn neon<C: ColorInterface>(self, high: C, low: C) {
        self.neon_with_speed(high, low, 3);
    }
    fn warn(self) {
        self.neon(RGB::new(226, 14, 14), RGB::new(158, 158, 158));
    }
}

pub trait ExtraColorInterface {
    fn grey0(self) -> CString;
}

impl<T> ExtraColorInterface for T where T: Colorful {
    fn grey0(self) -> CString { self.color(Color::Grey0) }
}

