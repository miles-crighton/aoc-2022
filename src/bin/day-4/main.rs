#[path = "../../utils.rs"]
mod utils;
use std::path::Path;
use utils::read_lines;

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

        if schedule_1[0] >= schedule_2[0] && schedule_1[1] <= schedule_2[1]
            || schedule_2[0] >= schedule_1[0] && schedule_2[1] <= schedule_1[1]
        {
            total_score += 1;
        }
    }

    println!("Total score: {:#?}", total_score)
}
