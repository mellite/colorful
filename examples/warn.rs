extern crate colorful;

use colorful::Colorful;

fn main() {
    let text = format!("{:^28}\n{}", "WARNING", "BIG BROTHER IS WATCHING YOU!!!");
    text.warn();
}
