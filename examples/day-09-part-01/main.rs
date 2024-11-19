use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let file = File::open("./examples/day-09-part-01/input-test.txt").expect("file not found");
    let file = File::open("./examples/day-09-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut acc = 0i32;
    for line in buff.lines() {
        let Some(line) = line
            .ok()
            .and_then(|x| if !x.is_empty() { Some(x) } else { None })
        else {
            continue;
        };
        acc += process_line(&line);
    }
    println!(">done processing lines!");
    println!("Result -> {}", acc);
}
fn process_line(s: &str) -> i32 {
    let history: Vec<i32> = s
        .split_whitespace()
        .map(|x| x.parse::<i32>().expect("expected to be a number"))
        .collect();
    let mut last:Vec<i32> = vec![history.last().copied().unwrap()];
    last.append(&mut recurse_dif(history));
    println!("last numbers {:?}",last);
    last.iter().sum()
}

fn recurse_dif(v:Vec<i32>) ->Vec<i32>{
    if v.iter().all(|x|matches!(x,0)){
        return vec![];
    }
    let mut next:Vec<i32> = Vec::new();
    for i in 1..v.len(){
        next.push(v[i]-v[i-1]);
    }
    let mut output = vec![next.last().copied().unwrap()];
    output.append(&mut recurse_dif(next));
    output
}
