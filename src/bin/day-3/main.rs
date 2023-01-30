#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

fn main() {
    let path = Path::new("src/bin/day-3/input.txt");
    let lines = read_lines(path);

    let mut total_score: i32 = 0;

    for line in lines {
        let line_data = line.unwrap();
        let split = line_data.split_at(line_data.len() / 2);

        'compartment_1: for i in split.0.chars() {
            for j in split.1.chars() {
                if i == j {
                    let code = i as i32;
                    if code >= 97 {
                        total_score += code - 96;
                    } else if code >= 65 {
                        total_score += code - 38;
                    }
                    break 'compartment_1;
                }
            }
        }
    }

    println!("Total score: {:#?}", total_score)
}
