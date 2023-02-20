#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

fn main() {
    let path = Path::new("src/bin/day-6/input.txt");

    let mut idx = 14;
    'outer: for win in read_lines(path)
        .next()
        .unwrap()
        .unwrap()
        .as_bytes()
        .windows(14)
    {
        for i in 1..win.len() {
            if win[i..].contains(&win[i - 1]) {
                idx += 1;
                continue 'outer;
            }
        }
        break;
    }

    println!("Sequence begins at: {:#?}", idx)
}
