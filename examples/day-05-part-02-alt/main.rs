use core::panic;
use std::{fs::File, io::Read};

fn main() {
    println!("Do not use this one, forward approach is too slow",);
    // let mut file = File::open("./examples/day-05-part-02-alt/input-test.txt").expect("file not found");
    let mut file = File::open("./examples/day-05-part-02-alt/input.txt").expect("file not found");
    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        panic!("error reading the file");
    }
    let mut seed_ranges: Vec<[i64; 2]> = vec![];
    let mut sections: Vec<Section> = vec![];
    for (i, part) in content.split("\n\n").enumerate() {
        if i == 0 {
            seed_ranges = process_header(part);
        } else {
            sections.push(process_section(part));
        }
    }
    println!("seed_ranges = {:?}", seed_ranges);
    let mut output: i64 = i64::MAX;
    for seed_range in seed_ranges.iter() {
        for seed in seed_range[0]..seed_range[1] {
            let mut current = seed;
            for section in sections.iter() {
                current = section.transform(current);
            }
            if current < output {
                output = current;
                println!("output -> {:>10}", output);
            }
        }
    }
    sections.iter().for_each(|x| {
        println!("map -> {:?}", x.name);
        x.rows.iter().for_each(|y| println!("   {y:>10?}"));
    });
    println!("Minimun location -> {}", output);
}
fn process_header(s: &str) -> Vec<[i64; 2]> {
    let mut header_split = s.split(":");
    let (Some(_), Some(seeds_pairs), None) = (
        header_split.next(),
        header_split.next().and_then(|x| {
            let output: Vec<i64> = x
                .split_whitespace()
                .filter(|n| !(n.is_empty()))
                .map(|n| n.parse::<i64>().expect("expected a number for seed"))
                .collect();
            Some(output)
        }),
        header_split.next(),
    ) else {
        unreachable!("there must be a header and seeds");
    };
    let mut seeds: Vec<[i64; 2]> = Vec::new();
    for i in 0..seeds_pairs.len() / 2 {
        let (Some(start), Some(range)) = (seeds_pairs.get(2 * i), seeds_pairs.get(2 * i + 1))
        else {
            return [].to_vec();
        };
        let range_of_pair: [i64; 2] = [*start, start + range];
        seeds.push(range_of_pair);
    }
    seeds
}

fn process_section(s: &str) -> Section {
    // let maps: [&str; 7] = [
    //     "seed-to-soil",
    //     "soil-to-fertilizer",
    //     "fertilizer-to-water",
    //     "water-to-light",
    //     "light-to-temperature",
    //     "temperature-to-humidity",
    //     "humidity-to-location",
    // ];
    let mut output = Section::new(Box::from(""), Vec::new());
    let mut section_split = s.split(" map:\n");
    let (Some(name), Some(body), None) = (
        section_split.next(),
        section_split.next(),
        section_split.next(),
    ) else {
        unreachable!("there must be a heading and a body for each section");
    };
    output.name = Box::from(name);
    for line in body.split('\n').filter(|x| !(x.is_empty())) {
        let mut line_split = line.split_whitespace();
        let (Some(dest), Some(src), Some(range), None) = (
            line_split.next().and_then(|x| x.parse::<i64>().ok()),
            line_split.next().and_then(|x| x.parse::<i64>().ok()),
            line_split.next().and_then(|x| x.parse::<i64>().ok()),
            line_split.next(),
        ) else {
            unreachable!("ther must be 3 numbers")
        };
        output.rows.push(SectionRow::new(dest, src, range));
    }
    output
}

#[derive(Debug)]
struct Section {
    name: Box<str>,
    rows: Vec<SectionRow>,
}
impl Section {
    fn new(name: Box<str>, rows: Vec<SectionRow>) -> Section {
        Section { name, rows }
    }
    fn transform(&self, n: i64) -> i64 {
        let mut output = n;
        for row in &self.rows {
            if (row.source..row.source + row.range).contains(&n) {
                output = n + row.destination - row.source;
            }
        }
        output
    }
}
#[derive(Debug)]
struct SectionRow {
    destination: i64,
    source: i64,
    range: i64,
}
impl SectionRow {
    fn new(dest: i64, src: i64, rang: i64) -> SectionRow {
        SectionRow {
            destination: dest,
            source: src,
            range: rang,
        }
    }
}
