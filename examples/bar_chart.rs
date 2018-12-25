extern crate colorful;

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
