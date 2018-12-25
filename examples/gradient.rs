extern crate colorful;

use colorful::Color;
use colorful::Colorful;

fn main() {
    println!("{}", "This code is editable and runnable!".gradient(Color::Red));
    println!("{}", "¡Este código es editable y ejecutable!".gradient(Color::Green));
    println!("{}", "Ce code est modifiable et exécutable !".gradient(Color::Yellow));
    println!("{}", "Questo codice è modificabile ed eseguibile!".gradient(Color::Blue));
    println!("{}", "このコードは編集して実行出来ます！".gradient(Color::Magenta));
    println!("{}", "여기에서 코드를 수정하고 실행할 수 있습니다!".gradient(Color::Cyan));
    println!("{}", "Ten kod można edytować oraz uruchomić!".gradient(Color::LightGray));
    println!("{}", "Este código é editável e executável!".gradient(Color::DarkGray));
    println!("{}", "Этот код можно отредактировать и запустить!".gradient(Color::LightRed));
    println!("{}", "Bạn có thể edit và run code trực tiếp!".gradient(Color::LightGreen));
    println!("{}", "这段代码是可以编辑并且能够运行的！".gradient(Color::LightYellow));
    println!("{}", "Dieser Code kann bearbeitet und ausgeführt werden!".gradient(Color::LightBlue));
    println!("{}", "Den här koden kan redigeras och köras!".gradient(Color::LightMagenta));
    println!("{}", "Tento kód můžete upravit a spustit".gradient(Color::LightCyan));
    println!("{}", "این کد قابلیت ویرایش و اجرا دارد!".gradient(Color::White));
    println!("{}", "โค้ดนี้สามารถแก้ไขได้และรันได้".gradient(Color::Grey0));
}
