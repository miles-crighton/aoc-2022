#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

fn main() {
    let path = Path::new("src/bin/day-1/input.txt");
    let lines = read_lines(path);

    let mut top_counts: [i32; 3] = [0, 0, 0];
    let mut current_count: i32 = 0;

    for line in lines {
        let data = line.unwrap();
        if data == "" {
            if current_count > top_counts[0] {
                top_counts[0] = current_count;
                top_counts.sort()
            }
            current_count = 0;
        } else {
            current_count += data.parse::<i32>().unwrap();
        }
    }

    println!(
        "Total calories for top 3 elves: {:#?}",
        top_counts.iter().sum::<i32>()
    )
}
