use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-08-part-02/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-08-part-02/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut node_map: HashMap<Box<str>, Node> = HashMap::new();
    let mut current: Vec<Box<str>> = Vec::new();
    let mut ends: Vec<Box<str>> = Vec::new();
    let mut accs: Vec<u64> = Vec::new();
    let mut found: Vec<bool> = Vec::new();
    for (i, line) in buff.lines().enumerate() {
        let Ok(line) = line else { continue };
        if line.is_empty() {
            continue;
        }
        if i == 0 {
            instructions = process_header(&line);
            println!(">done processing header!");
            continue;
        }
        let (key, val) = process_line(&line);
        if key.ends_with("A") {
            println!("found starting {:?}", &*key);
            current.push(key.clone());
            accs.push(0);
            found.push(false);
        }
        if key.ends_with("Z") {
            println!("found ending {:?}", &*key);
            ends.push(key.clone());
        }
        node_map.insert(key, val);
    }
    println!(">done processing lines!");
    for instruction in instructions.iter().cycle() {
        for (i, current_i) in current.iter_mut().enumerate() {
            if found[i] {
                continue;
            }
            accs[i] += 1;
            *current_i = node_map
                .get(current_i)
                .map(|x| match instruction {
                    Instruction::Left => x.0.clone(),
                    Instruction::Right => x.1.clone(),
                })
                .expect("node not in the map");
            if current_i.ends_with("Z") {
                found[i] = true;
            }
        }
        if found.iter().all(|x| *x) {
            break;
        }
    }
    println!("iterations per loop -> {:?}", accs);
    println!("Result -> {}", lcm_of_vec(accs));
}
fn process_header(s: &str) -> Vec<Instruction> {
    let mut output: Vec<Instruction> = Vec::new();
    for c in s.chars() {
        match c {
            'R' => output.push(Instruction::Right),
            'L' => output.push(Instruction::Left),
            _ => continue,
        }
    }
    output
}
fn process_line(s: &str) -> (Box<str>, Node) {
    let mut eq_split = s.split(" = ");
    let (Some(current), Some(next_tuple), None) =
        (eq_split.next(), eq_split.next(), eq_split.next())
    else {
        panic!("expected 2 members");
    };
    let mut next_split = next_tuple
        .trim_matches(|x| matches!(x, '(' | ')'))
        .split(", ");
    let (Some(left), Some(right), None) = (next_split.next(), next_split.next(), next_split.next())
    else {
        panic!("expected 2 members");
    };
    // println!(">{} = ({}, {})",current,left,right);
    (Box::from(current), Node(Box::from(left), Box::from(right)))
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    (a / gcd(a, b)) * b // Prevent overflow
}

fn lcm_of_vec(numbers: Vec<u64>) -> u64 {
    numbers.into_iter().reduce(|acc, n| lcm(acc, n)).unwrap_or(0)
}

enum Instruction {
    Left,
    Right,
}
struct Node(Box<str>, Box<str>);
