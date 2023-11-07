extern crate colorful;
extern crate core;

use colorful::Colorful;
use colorful::Color;
use colorful::core::ansi_support;
use colorful::Style;

#[test]
fn test_1() {
    assert_eq!("\u{1b}", "\x1B");
}

#[test]
fn test_no_ansi_support() {
    let s = "Hello world";

    // Test set:
    // `scenario` | `NO_COLOR`      | `CLICOLOR`       | `CLICOLOR_FORCE`   | colorize?
    // :--------- | :---------      | :---------       | :---------------   | :--------
    //      1     | unset/`== 0`    | unset            | unset              | true (default)
    //      2     | unset/`== 0`    | `!= 0`           | unset              | true
    //      3     | unset/`== 0`    | `== 0`           | unset              | false
    //      4     | unset/`== 0`    | any              | `== 0`             | false
    //      5     | unset/`== 0`    | any              | `!= 0`             | true
    //      6     | `!= 0`          | any              | any                | true
    // see https://bixense.com/clicolors/
    // see https://no-color.org/

    // Scenario 1:
    assert!(ansi_support());
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());

    // Scenario 2:
    std::env::set_var("NO_COLOR", "0");
    std::env::set_var("CLICOLOR", "XXX");
    assert!(ansi_support());
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());

    // Scenario 2:
    std::env::set_var("CLICOLOR", "XXX");
    assert!(ansi_support());
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());

    // Scenario 3:
    std::env::set_var("CLICOLOR", "0");
    assert!(!ansi_support());
    assert_eq!("Hello world".to_owned(), s.color(Color::Red).to_string());

    // Scenario 4:
    std::env::set_var("CLICOLOR", "XXX");
    std::env::set_var("CLICOLOR_FORCE", "0");
    assert!(!ansi_support());
    assert_eq!("Hello world".to_owned(), s.color(Color::Red).to_string());

    // Scenario 5:
    std::env::set_var("CLICOLOR_FORCE", "XXX");
    assert!(ansi_support());
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());

    // Scenario 5:
    std::env::set_var("NO_COLOR", "1");
    std::env::set_var("CLICOLOR", "XXX");
    std::env::set_var("CLICOLOR_FORCE", "XXX");
    assert!(!ansi_support());
    assert_eq!("Hello world".to_owned(), s.color(Color::Red).to_string());

    std::env::remove_var("NO_COLOR");
    std::env::remove_var("CLICOLOR");
    std::env::remove_var("CLICOLOR_FORCE");
    assert!(ansi_support());
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());
}

#[test]
fn test_color() {
    let s = "Hello world";
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Red).to_string());
    assert_eq!("\x1B[38;5;220mHello world\x1B[0m".to_owned(), s.color(Color::Red).color(Color::Gold1).to_string());
}

#[test]
fn test_bg_color() {
    let s = "Hello world";
    assert_eq!("\x1B[38;5;1m\x1B[48;5;16mHello world\x1B[0m".to_owned(), s.color(Color::Red).bg_color(Color::Grey0).to_string());
    assert_eq!("\x1B[38;5;1m\x1B[48;5;6mHello world\x1B[0m".to_owned(), s.color(Color::Red).bg_cyan().to_string());
    assert_eq!("\x1B[38;5;220m\x1B[48;5;6mHello world\x1B[0m".to_owned(), s.color(Color::Red).color(Color::Gold1).bg_color(Color::Cyan).to_string());
}


#[test]
fn test_style() {
    let s = "Hello world";
    assert_eq!("\x1B[1mHello world\x1B[0m".to_owned(), s.style(Style::Bold).to_string());
    assert_eq!("\x1B[1;5mHello world\x1B[0m".to_owned(), s.style(Style::Bold).style(Style::Blink).to_string());
}

#[test]
fn test_interface() {
    let s = "Hello world";
    assert_eq!("\x1B[1mHello world\x1B[0m".to_owned(), s.bold().to_string());
    assert_eq!("\x1B[1;5mHello world\x1B[0m".to_owned(), s.bold().blink().to_string());
    assert_eq!("\x1B[38;5;1mHello world\x1B[0m".to_owned(), s.red().to_string());
    assert_eq!("\x1B[38;5;2mHello world\x1B[0m".to_owned(), s.red().green().to_string());
}

#[test]
fn test_mix() {
    let s = "Hello world";
    assert_eq!("\x1B[38;5;1;5mHello world\x1B[0m".to_owned(), s.color(Color::Red).blink().to_string());
    assert_eq!("\x1B[38;5;220;1mHello world\x1B[0m".to_owned(), s.bold().color(Color::Gold1).to_string());

    assert_eq!("\x1B[38;5;2;5;1mHello world\x1B[0m".to_owned(), s.color(Color::Green).blink().bold().to_string());
    assert_eq!("\x1B[38;5;220;1;5mHello world\x1B[0m".to_owned(), s.bold().blink().color(Color::Gold1).to_string());
}
