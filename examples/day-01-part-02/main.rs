use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/day-01-part-02/input.txt").expect("file not found");
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
    let mut first_index = usize::MAX;
    let mut last_index = 0;
    let mut first: u32 = 0;
    let mut last: u32 = 0;
    let map_digits = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];
    for (i, n) in map_digits.iter().enumerate() {
        if let Some(x) = s.find(n) {
            if x < first_index {
                // println!("found -> {n:}");
                first_index = x;
                first = (i as u32) % 10;
                // println!("first -> {first:}");
            }
        }
        if let Some(x) = s.rfind(n) {
            if x >= last_index {
                // println!("found -> {n:}");
                last_index = x;
                last = (i as u32) % 10;
                // println!("last  -> {last:}");
            }
        }
    }
    first * 10 + last
}
