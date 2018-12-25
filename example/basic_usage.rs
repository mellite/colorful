extern crate colorful;

use colorful::Color;
use colorful::Colorful;
use colorful::HSL;
use colorful::RGB;

fn main() {
    let s = "Hello world";
    println!("{}", s.color(Color::Blue).bg_color(Color::Yellow).bold());
    println!("{}", s.color(HSL::new(1.0, 1.0, 0.5)).bold());
    println!("{}", s.color(RGB::new(255, 0, 0)).bold());
    println!("{}", s.blue().bg_yellow());
}