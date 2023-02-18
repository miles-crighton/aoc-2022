#[path = "../../utils.rs"]
mod utils;
use itertools::Itertools;
use std::path::Path;
use utils::read_lines;

fn main() {
    let path = Path::new("src/bin/day-6/input.txt");

    let mut idx = 4;
    for (a, b, c, d) in read_lines(path)
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .tuple_windows()
    {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            break;
        }
        idx += 1;
    }

    println!("Sequence begins at: {:#?}", idx)
}
