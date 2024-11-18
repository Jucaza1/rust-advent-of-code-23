use std::{fs::File, io::Read};

fn main() {
    // let mut file = File::open("./examples/day-06-part-02/input-test.txt").expect("file not found");
    let mut file = File::open("./examples/day-06-part-02/input.txt").expect("file not found");
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
            .replace(" ","")
            .split(":")
            .filter_map(|x| x.parse::<u64>().ok())
            .collect(),
        distance: distance_row
            .replace(" ","")
            .split(":")
            .filter_map(|x| x.parse::<u64>().ok())
            .collect(),
    };
    let output = proces_col(data.time.first().unwrap_or(&0),data.distance.first().unwrap_or(&0));
    println!("Result -> {}",output);
}

struct RecordData {
    time: Vec<u64>,
    distance: Vec<u64>,
}
fn proces_col(t: &u64, d: &u64) -> u64 {
    let mut count = 0u64;
    for i in 1..*t{
        if i*(t-i) > *d{
            count += 1;
        }
    }
    println!("ways to beat the record: {count}");
    count
}
