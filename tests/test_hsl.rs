extern crate colorful;
extern crate core;

use colorful::Colorful;
use colorful::HSL;
use colorful::hsl_to_rgb;
use colorful::RGB;

#[test]
fn test_hsl_color() {
    let s = "Hello world";
    assert_eq!("\x1B[38;2;19;205;94mHello world\x1B[0m", s.hsl(0.4, 0.83, 0.44).to_string());
}

#[test]
fn test_hsl_2_rgb_1() {
    let hsl = HSL::new(0.7, 0.50, 0.60);
    let rgb = RGB::new(122, 102, 204);

    assert_eq!(rgb, hsl_to_rgb(hsl));
}

#[test]
fn test_hsl_2_rgb_2() {
    let hsl = HSL::new(0.7, 0.0, 0.60);
    let rgb = RGB::new(153, 153, 153);
    assert_eq!(rgb, hsl_to_rgb(hsl));
}

#[test]
fn test_hsl_2_rgb_3() {
    let hsl = HSL::new(0.7, 0.50, 0.30);
    let rgb = RGB::new(54, 38, 115);

    assert_eq!(rgb, hsl_to_rgb(hsl));
}