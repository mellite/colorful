<h1 align="center">
	<a>Colorful</a>
	<br>
	<br>
</h1>

[![Build Status](https://travis-ci.org/da2018/colorful.svg?branch=master)](https://travis-ci.org/da2018/colorful) [![Coverage Status](https://img.shields.io/coveralls/github/da2018/colorful.svg)](https://coveralls.io/github/da2018/colorful?branch=master)

## Usage

### bar chart

``` Rust
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

- HSL support
- RGB support
- Gradient mode
- Rainbow mode
- brackets mode


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fda2018%2Fcolorful.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fda2018%2Fcolorful?ref=badge_large)