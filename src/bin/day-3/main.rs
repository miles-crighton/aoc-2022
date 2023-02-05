#[path = "../../utils.rs"]
mod utils;
use itertools::Itertools;
use std::path::Path;
use utils::read_lines;

fn main() {
    let path = Path::new("src/bin/day-3/input.txt");
    let lines = read_lines(path);

    let mut total_score: i32 = 0;

    for chunk in lines.tuples::<(_, _, _)>() {
        let l1 = chunk.0.unwrap();
        let l2 = chunk.1.unwrap();
        let l3 = chunk.2.unwrap();

        'line_1: for i in l1.chars() {
            for j in l2.chars() {
                for k in l3.chars() {
                    if i == j && j == k {
                        let code = i as i32;
                        if code >= 97 {
                            total_score += code - 96;
                        } else if code >= 65 {
                            total_score += code - 38;
                        }
                        break 'line_1;
                    }
                }
            }
        }
    }

    println!("Total score: {:#?}", total_score)
}
