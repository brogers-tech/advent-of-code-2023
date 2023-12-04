use std::fs;
use std::io::{BufRead, BufReader};


pub fn read_file(path: &str) -> impl Iterator<Item=String> {
    let input = fs::File::open(path).expect(format!("File {} not found.", path).as_str());
    let buffered = BufReader::new(input);

    buffered
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| l.trim_end().to_string())
}
