extern crate colorful;

use colorful::Color;
use colorful::Colorful;

fn main() {
    println!("{}", "This code is editable and runnable!\n这段代码是可以编辑并且能够运行的！".gradient(Color::Red));
}