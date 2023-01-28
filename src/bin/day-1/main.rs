use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn read_lines(filename: &Path) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let path = Path::new("src/bin/day-1/input.txt");
    let lines = read_lines(path);

    let mut max_count: i32 = 0;
    let mut current_count: i32 = 0;

    for line in lines {
        let data = line.unwrap();
        if data == "" {
            if current_count > max_count {
                max_count = current_count
            }
            current_count = 0;
        } else {
            current_count += data.parse::<i32>().unwrap();
        }
    }

    println!("Max count: {}", max_count)
}
