use std::io;
use std::io::Write;
use std::num::ParseIntError;

struct Color {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let colors: [&str; 3] = ["red", "green", "blue"];
    let mut v: Vec<u32> = vec![];

    for color in colors.iter() {
        let mut buffer = String::new();
        print!("Enter a number of {} : ", color);
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        stdin.read_line(&mut buffer).expect("Error getting guess");

        let input = parse_num(&mut buffer);
        let input = match input {
            Ok(n) => n,
            Err(_) => {
                panic!("No number")
            }
        };

        v.push(input);
    }

    let r = Color {
        red: v[0],
        green: v[1],
        blue: v[2],
    };

    // ANSI escape sequence
    println!("\x1b[38;2;{0};{1};{2}m■\x1b[m", r.red, r.green, r.blue);
}

fn parse_num(s: &String) -> Result<u32, ParseIntError> {
    s.trim().parse()
}
