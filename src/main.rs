// Maybe, this assertion should be after assert_matches become a stable feature.
// #![feature(assert_matches)]
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    /// new() returns a new Color instance.
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    /// ansi() returns a string for showing this color on terminal.
    pub fn ansi(&self) -> String {
        format!("\x1b[38;2;{0};{1};{2}m", self.red, self.green, self.blue)
    }

    /// reset_ansi() returns a constant string for resetting ANSI escape sequence for terminal colors.
    pub fn reset_ansi() -> &'static str {
        "\x1b[m"
    }
}

fn parse_num(s: &str) -> Result<u8, ParseIntError> {
    s.trim().parse()
}

/// read_with shows a message to encourage input a line.
fn read_with(msg: &str) -> String {
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error getting guess");
    buffer
}

fn read_color() -> Result<Color, ParseIntError> {
    Ok(Color {
        red: parse_num(&read_with("Enter a number of red"))?,
        blue: parse_num(&read_with("Enter a number of blue"))?,
        green: parse_num(&read_with("Enter a number of green"))?,
    })
}

fn main() {
    let c = read_color().expect("Wrong color value input");
    println!("{}■{}", c.ansi(), Color::reset_ansi());
}

#[cfg(test)]
mod tests {
    use super::*;
    // Maybe, this assertion should be after assert_matches become a stable feature.
    // use std::assert_matches::assert_matches;

    #[test]
    fn test_ansi() {
        assert_eq!("\x1b[38;2;0;0;0m", Color::new(0, 0, 0).ansi());
        assert_eq!("\x1b[38;2;255;0;0m", Color::new(255, 0, 0).ansi());
        assert_eq!("\x1b[38;2;0;255;0m", Color::new(0, 255, 0).ansi());
        assert_eq!("\x1b[38;2;0;0;255m", Color::new(0, 0, 255).ansi());
    }
    #[test]
    fn test_parse_num() {
        assert_eq!(parse_num("0"), Ok(0));
        assert_eq!(parse_num("255"), Ok(255));
        // Maybe, this assertion should be after assert_matches become a stable feature.
        // assert_matches!(parse_num("-1"), Err(_));
        // assert_matches!(parse_num("256"), Err(_));
    }
}
