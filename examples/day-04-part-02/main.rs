use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./examples/day-04-part-02/input.txt").expect("file not found");
    let buf = BufReader::new(file);
    let mut times_each_card: Vec<u32> = Vec::new();
    for (i, line) in buf.lines().enumerate() {
        if times_each_card.get(i).is_none() {
            // 0 -> 1
            times_each_card.push(1);
        } else {
            // x -> x + 1
            times_each_card[i] += 1;
        }
        let Ok(line) = line else {
            return;
        };
        let res: usize = process_line(line) as usize;
        for _ in 0..times_each_card[i] {
            for j in 1..=res {
                match times_each_card.get(i + j) {
                    None => times_each_card.push(1),
                    Some(x) => times_each_card[i + j] = x + 1,
                }
            }
        }
    }
    println!("Result -> {}", times_each_card.iter().sum::<u32>());
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
    acc
}
