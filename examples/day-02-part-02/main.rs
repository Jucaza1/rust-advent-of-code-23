use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./examples/day-02-part-02/input.txt").expect("file not found");
    let buf = BufReader::new(file);
    let mut acc = 0;
    for line in buf.lines() {
        let Ok(line) = line else {
            break;
        };
        acc += process_line(line);
    }
    println!("Result -> {acc:}");
    Ok(())
}

fn process_line(s: String) -> u32 {
    // let mut res = 0;
    let mut color_max = (0, 0, 0);
    // let color_str = ("red","green","blue");
    let mut splited_itr = s.split(":");
    let (Some(game_str), Some(rounds_str), None) =
        (splited_itr.next(), splited_itr.next(), splited_itr.next())
    else {
        return 0;
    };

    let mut splited_game = game_str.split(" ");
    let (Some(_), Some(_), None) = (
        splited_game.next(),
        splited_game.next(),
        splited_game.next(),
    ) else {
        return 0;
    };

    for color_num in rounds_str.split(";") {
        for pair in color_num.split(",") {
            let mut tup_iter = pair.trim().split(" ");
            let (Some(n_color_str), Some(color), None) =
                (tup_iter.next(), tup_iter.next(), tup_iter.next())
            else {
                return 0;
            };
            match (color, n_color_str.parse::<u32>()) {
                ("red", Ok(n_red)) if n_red > color_max.0 => color_max.0 = n_red,
                ("green", Ok(n_green)) if n_green > color_max.1 => color_max.1 = n_green,
                ("blue", Ok(n_blue)) if n_blue > color_max.2 => color_max.2 = n_blue,
                (_, _) => {},
            }
        }
    }

    color_max.0 * color_max.1 * color_max.2
}
