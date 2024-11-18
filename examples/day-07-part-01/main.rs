use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./examples/day-07-part-01/input-test.txt").expect("file not found");
    // let file = File::open("./examples/day-07-part-01/input.txt").expect("file not found");
    let buff = BufReader::new(file);
    let mut rows: Vec<Row> = Vec::new();
    for line in buff.lines() {
        let Ok(line) = line else { continue };
        rows.push(process_line(&line));
    }
    //smallest to greatest
    rows.sort();
    let mut output = 0;
    for (i, row) in rows.iter().enumerate() {
        // println!("{}*{}",row.bid,i+1);
        output += row.bid * (i + 1) as u32;
    }
    println!("Result -> {}", output);
}
fn process_line(s: &str) -> Row {
    let card_map: HashMap<char, u8> = [
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]
    .into_iter()
    .collect();
    let mut line_split = s.split(" ");
    let (Some(hand), Some(bid), None) = (
        line_split.next(),
        line_split.next().and_then(|x| x.parse::<u32>().ok()),
        line_split.next(),
    ) else {
        panic!("expected only 2 groups");
    };
    let mut card_rep_map: HashMap<u8, u8> = HashMap::new();
    let mut cards_values: Vec<u8> = Vec::new();
    for card in hand.chars() {
        let Some(n) = card_map.get(&card) else {
            panic!("not a card")
        };
        card_rep_map.entry(*n).and_modify(|x| *x += 1).or_insert(1);
        cards_values.push(*n);
    }
    let mut state = HandKind::High;
    for (_, val) in card_rep_map {
        state = match (&state, val) {
            (_, 1) => state,
            (HandKind::High, 2) => HandKind::Pair,
            (HandKind::High, 3) => HandKind::Three,
            (_, 4) => HandKind::Four,
            (_, 5) => HandKind::Five,
            (HandKind::Pair, 2) => HandKind::Double,
            (HandKind::Pair, 3) => HandKind::House,
            (HandKind::Three, 2) => HandKind::House,
            (_, _) => state,
        }
    }
    Row {
        kind: state,
        values: cards_values,
        bid,
    }
}
#[derive(PartialEq, Eq, Debug)]
struct Row {
    kind: HandKind,
    values: Vec<u8>,
    bid: u32,
}
impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (i, val) in self.values.iter().enumerate() {
                    match val.cmp(other.values.get(i).unwrap_or(&0)) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                    }
                }
                Ordering::Equal
            }
        }
    }
}
impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    Five = 7,
    Four = 6,
    House = 5,
    Three = 4,
    Double = 3,
    Pair = 2,
    High = 1,
}
const CARD_MAP: [(&str, u8); 13] = [
    ("A", 13),
    ("K", 12),
    ("Q", 11),
    ("J", 10),
    ("T", 9),
    ("9", 8),
    ("8", 7),
    ("7", 6),
    ("6", 5),
    ("5", 4),
    ("4", 3),
    ("3", 2),
    ("2", 1),
];
