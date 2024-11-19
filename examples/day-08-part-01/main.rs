use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-08-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-08-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut node_map: HashMap<Box<str>, Node> = HashMap::new();
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
        node_map.insert(key, val);
    }
    println!(">done processing lines!");
    let mut acc = 0;
    //Important to start at "AAA"
    let mut current: Box<str> = Box::from("AAA");
    for instruction in instructions.iter().cycle() {
        acc += 1;
        current = node_map
            .get(&current)
            .map(|x| match instruction {
                Instruction::Left => x.0.clone(),
                Instruction::Right => x.1.clone(),
            })
            .expect("node not in the map");
        println!("current = {}", &*current);
        if *current == *"ZZZ" {
            break;
        }
    }
    println!("Result -> {}", acc);
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
enum Instruction {
    Left,
    Right,
}
struct Node(Box<str>, Box<str>);
