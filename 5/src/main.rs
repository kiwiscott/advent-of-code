use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let _p = all_lines("data.txt");
}

pub fn all_lines(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let v: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    return v;
}

#[test]
fn testx() {
    assert!(true);
}
