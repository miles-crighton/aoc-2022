#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

fn calc_match_outcome_score(choices: (char, char)) -> i32 {
    match choices.0 {
        'A' => match choices.1 {
            'X' => 3,
            'Y' => 4,
            'Z' => 8,
            _ => panic!("Unexpected char in strategy choices"),
        },
        'B' => match choices.1 {
            'X' => 1,
            'Y' => 5,
            'Z' => 9,
            _ => panic!("Unexpected char in strategy choices"),
        },
        'C' => match choices.1 {
            'X' => 2,
            'Y' => 6,
            'Z' => 7,
            _ => panic!("Unexpected char in strategy choices"),
        },
        _ => panic!("Unexpected char in opponent choices"),
    }
}

fn main() {
    let path = Path::new("src/bin/day-2/input.txt");
    let lines = read_lines(path);

    let mut total_score: i32 = 0;

    for line in lines {
        let line_data = line.unwrap();
        let mut split = line_data.split_whitespace();
        let choices = (
            // Looks fine chief...
            split.next().unwrap().chars().next().unwrap(),
            split.next().unwrap().chars().next().unwrap(),
        )
            .to_owned();

        total_score += calc_match_outcome_score(choices);
    }

    println!("Total score: {:#?}", total_score)
}
