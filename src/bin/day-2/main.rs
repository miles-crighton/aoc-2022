#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

fn calc_match_outcome_score(choices: (char, char)) -> i32 {
    match choices.0 {
        'A' => match choices.1 {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            _ => panic!("Unexpected char in us choices"),
        },
        'B' => match choices.1 {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Unexpected char in us choices"),
        },
        'C' => match choices.1 {
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            _ => panic!("Unexpected char us choices"),
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

        match choices.1 {
            'X' => total_score += 1,
            'Y' => total_score += 2,
            'Z' => total_score += 3,
            _ => println!("Unexpected char in choices"),
        }

        total_score += calc_match_outcome_score(choices);
    }

    println!("Total score: {:#?}", total_score)
}
