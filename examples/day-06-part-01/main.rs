use std::{fs::File, io::Read};

fn main() {
    // let mut file = File::open("./examples/day-06-part-01/input-test.txt").expect("file not found");
    let mut file = File::open("./examples/day-06-part-01/input.txt").expect("file not found");
    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        return;
    };
    let mut content_split = content.split("\n");
    let (Some(time_row), Some(distance_row), None) = (
        content_split.next().filter(|x|!x.is_empty()),
        content_split.next().filter(|x|!x.is_empty()),
        content_split.next().filter(|x|!x.is_empty()),
    ) else {
        panic!("expected only 2 rows");
    };
    let data = RecordData {
        time: time_row
            .split(" ")
            .filter_map(|x| x.parse::<u32>().ok())
            .collect(),
        distance: distance_row
            .split(" ")
            .filter_map(|x| x.parse::<u32>().ok())
            .collect(),
    };
    let mut acc = 1u32;
    for i in 0..data.time.len(){
        acc *= proces_col(data.time.get(i).unwrap_or(&0),data.distance.get(i).unwrap_or(&0));
    }
    println!("Result -> {}",acc);
}

struct RecordData {
    time: Vec<u32>,
    distance: Vec<u32>,
}
fn proces_col(t: &u32, d: &u32) -> u32 {
    let mut count = 0u32;
    for i in 1..*t{
        if i*(t-i) > *d{
            count += 1;
        }
    }
    println!("ways to beat the record: {count}");
    count
}
