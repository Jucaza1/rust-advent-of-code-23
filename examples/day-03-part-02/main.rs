use std::{
    collections::HashMap, fs::File, io::{BufRead, BufReader}
};

fn main() {
    let file = File::open("./examples/day-03-part-02/input.txt").expect("file not found");
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

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point(usize,usize);

fn process_matrix(mat: Vec<Vec<char>>) -> u32 {
    let mut gear_map : HashMap<Point,Vec<u32>> = HashMap::new();
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
                if let Some(point) = is_next_to_symbol(&mat, i, j, k) {
                    gear_map.entry(point).and_modify(|prev| prev.push(num)).or_insert(vec![num]);
                }
                j += k - 1;
            }
            j += 1;
        }
    }
    gear_map.values().filter(|v| v.len()==2).map(|v| v[0]*v[1]).sum()
}

fn is_next_to_symbol(mat: &[Vec<char>], i: usize, j: usize, length: usize) -> Option<Point> {
    let (mut t, mut k) = (i.saturating_sub(1), j.saturating_sub(1));
    let mut res = None;
    while t < i + 2 {
        while k < j + length + 1 {
            let valid_index = t < mat.len() && k < mat[i].len();
            let inside_number = t == i && k >= j && k < j + length;
            let is_symbol = valid_index && !inside_number && mat[t][k] == '*';

            if is_symbol {
                res = Some(Point(t,k));
            }
            k += 1;
        }
        k = j.saturating_sub(1);
        t += 1;
    }
    res
}
