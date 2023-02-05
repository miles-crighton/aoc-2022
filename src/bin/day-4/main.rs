#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

fn within_range(r1: &Vec<i32>, r2: &Vec<i32>) -> bool {
    return r1[0] >= r2[0] && r1[0] <= r2[1] || r1[1] >= r2[0] && r1[1] <= r2[1];
}

fn main() {
    let path = Path::new("src/bin/day-4/input.txt");
    let lines = read_lines(path);

    let mut total_score: i32 = 0;

    for line in lines {
        let line_data = line.unwrap();
        let schedules: Vec<&str> = line_data.split(",").collect();
        let schedule_1: Vec<i32> = schedules[0]
            .split("-")
            .map(|a| a.parse().unwrap())
            .collect();
        let schedule_2: Vec<i32> = schedules[1]
            .split("-")
            .map(|a| a.parse().unwrap())
            .collect();

        if within_range(&schedule_1, &schedule_2) || within_range(&schedule_2, &schedule_1) {
            total_score += 1;
        }
    }

    println!("Total score: {:#?}", total_score)
}
