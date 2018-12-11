extern crate colorful;

use colorful::Color;
use colorful::Colorful;

fn main() {
    println!("{}", "言葉にできず　凍えたままで 人前ではやさしく生きていた しわよせで　こんなふうに雑に 雨の夜にきみを　抱きしめてた".rainbow().underlined());
}