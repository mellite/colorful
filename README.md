<h1 align="center">
        <a>Colorful</a>
        <br>
        <br>
</h1>

[![Build Status](https://travis-ci.org/rocketsman/colorful.svg?branch=master)](https://travis-ci.org/rocketsman/colorful) [![Coverage Status](https://coveralls.io/repos/github/rocketsman/colorful/badge.svg?branch=master)](https://coveralls.io/github/rocketsman/colorful?branch=master) [![Codacy Badge](https://api.codacy.com/project/badge/Grade/37a45510f41445eea0168f0f07e8f7cb)](https://app.codacy.com/app/rocketsman/colorful_2?utm_source=github.com&utm_medium=referral&utm_content=rocketsman/colorful&utm_campaign=Badge_Grade_Dashboard)

## Usage

### Basic Usage

```Rust
extern crate colorful;

use colorful::Color;
use colorful::Colorful;

fn main() {
    let s = "Hello world";
    println!("{}", s.color(Color::Blue).bg_color(Color::Yellow).bold());
    println!("{}", s.blue().bg_yellow());
}
```

### bar chart

```Rust
extern crate colorful;

use std::cmp;

use colorful::Colorful;

fn main() {
    let s = " ";
    println!("{}\n", "Most Loved, Dreaded, and Wanted Languages".red());
    let values = vec![78.9, 75.1, 68.0, 67.0, 65.6, 65.1, 61.9, 60.4];
    let languages = vec!["Rust", "Kotlin", "Python", "TypeScript", "Go", "Swift", "JavaScript", "C#"];
    let c = languages.iter().max_by_key(|x| x.len()).unwrap();

    for (i, value) in values.iter().enumerate() {
        let h = (*value as f32 * 15.0 % 360.0) / 360.0;
        let length = (value - 30.0) as usize;
        println!("{:<width$} | {} {}%\n", languages.get(i).unwrap(), s.repeat(length).bg_hsl(h, 0.83, 0.44), value, width = c.len());
    }
}
```

Output

<div>
    <img src="https://pic1.zhimg.com/v2-0306c2f7eb1073fe3917684feb73bc5e.png" width="500px"</img>
</div>

## Todo

-   [x] Basic 16 color
-   [x] Extra 240 color
-   [x] HSL support
-   [x] RGB support
-   [ ] Gradient mode
-   [ ] Animation mode

## License

[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fda2018%2Fcolorful.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fda2018%2Fcolorful?ref=badge_large)
