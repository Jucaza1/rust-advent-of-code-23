use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/day-01-part-01/input.txt").expect("file not found");
    let buf = BufReader::new(file);
    let mut acc = 0;
    for line in buf.lines() {
        let Ok(line) = line else {
            break;
        };
        acc += process_line(line);
    }
    println!("Result -> {acc:}");
    Ok(())
}

fn process_line(s: String) -> u32 {
    let mut first = 0;
    let mut last = 0;
    for c in s.chars() {
        if c.is_ascii_digit() {
            first = c.to_digit(10).unwrap_or(0);
            break;
        }
    }
    for c in s.chars().rev() {
        if c.is_ascii_digit() {
            last = c.to_digit(10).unwrap_or(0);
            break;
        }
    }
    first * 10 + last
}
