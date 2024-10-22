use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./examples/day-03-part-01/input.txt").expect("file not found");
    let buf = BufReader::new(file);
    // let mut acc = 0;
    let mut mat: Vec<Vec<char>> = Vec::new();
    for line in buf.lines() {
        let Ok(line) = line else {
            return;
        };
        mat.push(line.chars().collect());
    }
    let result = process_matrix(mat);
    println!("Result -> {result:}");
}

fn process_matrix(mat: Vec<Vec<char>>) -> u32 {
    let mut acc = 0;
    for (i, _) in mat.iter().enumerate() {
        let mut j = 0;
        while j < mat[i].len() {
            let mut num = 0;
            let mut k = 0; //k will be the length of the number too
            if mat[i][j].is_ascii_digit() {
                while j + k < mat[i].len() && mat[i][j + k].is_ascii_digit() {
                    num = num * 10 + mat[i][j + k].to_digit(10).unwrap();
                    k += 1;
                }
                if is_next_to_symbol(&mat, i, j, k) {
                    acc += num;
                }
                j += k - 1;
            }
            j += 1;
        }
    }
    acc
}

fn is_next_to_symbol(mat: &[Vec<char>], i: usize, j: usize, length: usize) -> bool {
    let (mut t, mut k) = (i.saturating_sub(1), j.saturating_sub(1));
    let mut res = false;
    while t < i + 2 {
        while k < j + length + 1 {
            let valid_index = t < mat.len() && k < mat[i].len();
            let inside_number = t == i && k >= j && k < j + length;
            let is_symbol = valid_index && !inside_number && mat[t][k] != '.';

            if is_symbol {
                res = true;
            }
            k += 1;
        }
        k = j.saturating_sub(1);
        t += 1;
    }
    res
}
