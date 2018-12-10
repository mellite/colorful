extern crate colorful;

use colorful::Color;
use colorful::Colorful;

fn main() {
    let s = "Hello world";
    println!("{}", s.color(Color::Blue).bg_color(Color::Yellow).bold());
    println!("{}", s.blue().bg_yellow());
}