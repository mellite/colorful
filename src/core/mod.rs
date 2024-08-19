use std::env;
use core::colors::Colorado;
use core::style::Style;
use HSL;


pub mod colors;
pub mod symbols;
pub mod style;
pub mod color_string;
pub mod rgb;
pub mod hsl;

/// Reads environment variables to determine whether colorization should
/// be used or not. `CLICOLOR_FORCE` takes highest priority, followed by
/// `NO_COLOR`, followed by `CLICOLOR`. In the absence of manual overrides,
/// which take precedence over all environment variables, the priority
/// of these variables can be expressed as follows.
///
/// `NO_COLOR`      | `CLICOLOR`       | `CLICOLOR_FORCE`   | colorize?
/// :---------      | :---------       | :---------------   | :--------
/// unset/`== 0`    | unset            | unset              | true (default)
/// unset/`== 0`    | `!= 0`           | unset              | true
/// unset/`== 0`    | `== 0`           | unset              | false
/// unset/`== 0`    | any              | `== 0`             | false
/// unset/`== 0`    | any              | `!= 0`             | true
/// `!= 0`          | any              | any                | true
/// see https://bixense.com/clicolors/
/// see https://no-color.org/
pub fn ansi_support() -> bool {
    // NO_COLOR unset or "0" => explicit do not support ansi
    match env::var("NO_COLOR").as_deref() {
        Ok("0") | Err(_) => true,
        Ok(_) => return false,
    };
    // CLICOLOR_FORCE is "0" => explicit do not support ansi
    // CLICOLOR_FORCE is "1" => explicit support ansi
    match env::var("CLICOLOR_FORCE").as_deref() {
        Ok("0") => return false,
        Ok(_) => return true,
        Err(_) => true,
    };
    // CLICOLOR is "0" => explicit do not support ansi
    // CLICOLOR is "1" => explicit support ansi
    match env::var("CLICOLOR").as_deref() {
        Ok("0") => return false,
        Ok(_) => return true,
        Err(_) => true,
    };
    true
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

impl StrMarker for String {
    fn to_str(&self) -> String {
        self.clone()
    }
}

/// `ColorInterface` is for basic trait for `Colorful`, `RGB`, `HSL` and `Color` implement this trait.
pub trait ColorInterface: Clone {
    fn to_color_str(&self) -> String;
    fn to_hsl(&self) -> HSL;
}
