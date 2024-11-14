use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./examples/day-04-part-01/input.txt").expect("file not found");
    let buf = BufReader::new(file);
    let mut acc = 0;
    for line in buf.lines() {
        let Ok(line) = line else {
            return;
        };
        acc += process_line(line);
    }
    println!("Result -> {acc:}");
}

fn process_line(s: String) -> u32 {
    let mut splited_line_iter = s.split(":");
    let (Some(_card_str), Some(numbers_str), None) = (
        splited_line_iter.next(),
        splited_line_iter.next(),
        splited_line_iter.next(),
    ) else {
        return 0;
    };
    let mut splited_numbers = numbers_str.split("|");
    let (Some(winner_str), Some(current_str), None) = (
        splited_numbers.next(),
        splited_numbers.next(),
        splited_numbers.next(),
    ) else {
        return 0;
    };
    let winner_numbers: Vec<u32> = winner_str
        .trim()
        .replace("  ", " ") // single digits have 2 spaces
        .split(" ")
        .map(|s| s.parse::<u32>().expect("NaN"))
        .collect();
    let current_numbers: Vec<u32> = current_str
        .trim()
        .replace("  ", " ") // single digits have 2 spaces
        .split(" ")
        .map(|s| s.parse::<u32>().expect("NaN"))
        .collect();
    let mut acc = 0;
    for w in winner_numbers {
        for c in &current_numbers {
            if w == *c {
                acc += 1;
            }
        }
    }
    if acc == 0{
        return 0;
    }
    2_u32.pow(acc-1)
}
