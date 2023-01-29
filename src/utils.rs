use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn read_lines(filename: &Path) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}
