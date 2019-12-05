use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(filename: &str) -> Vec<String> {
    let file = format!("input/{}.txt", filename);
    let file = File::open(file).expect("failed to read file");
    let reader = BufReader::new(file);

    let mut lines = vec![];
    for line in reader.lines() {
        lines.push(line.expect("failed to read line"))
    }
    lines
}
