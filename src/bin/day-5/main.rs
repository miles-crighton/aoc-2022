#[path = "../../utils.rs"]
mod utils;
use regex::Regex;
use std::path::Path;
use utils::read_lines;

fn main() {
    let mut lines = read_lines(Path::new("src/bin/day-5/input.txt"));

    // Extract creates from lines
    let mut crates: Vec<Vec<char>> = Vec::new();
    let mut expected_length = 0;
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.contains(" 1 ") {
            break;
        }
        if expected_length == 0 {
            expected_length = (line.len() / 4) + 1;
        } else if expected_length != (line.len() / 4) + 1 {
            panic!("Crates line has inconsistent length")
        }
        for i in 0..expected_length {
            let val = line.chars().nth((i * 4) + 1).unwrap();
            if crates.get(i).is_none() {
                if val.is_whitespace() {
                    crates.push(Vec::new());
                } else {
                    crates.push(vec![val]);
                }
            } else {
                if !val.is_whitespace() {
                    crates[i].push(val);
                }
            }
        }
    }

    // Reverse vectors
    crates = crates
        .into_iter()
        .map(|row| row.into_iter().rev().collect())
        .collect();

    // Skip empty line
    lines.next();

    // Exract operations from lines
    for line in lines {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        for cap in re.captures_iter(&line.unwrap().to_string()) {
            let from: usize = cap[2].parse().unwrap();
            let to: usize = cap[3].parse().unwrap();
            let count: i32 = cap[1].parse().unwrap();

            let mut crates_to_move = Vec::new();
            for _ in 0..count {
                crates_to_move.push(crates[from - 1].pop().unwrap())
            }
            for crate_to_move in crates_to_move.into_iter().rev().collect::<Vec<char>>() {
                crates[to - 1].push(crate_to_move);
            }
        }
    }

    print!("Result: ");
    for row in crates {
        print!("{}", &row.last().unwrap())
    }
}
