use aoc::bag_map::BagMap;

fn main() {
    let lines = all_lines("input.txt");
    let bg = BagMap::new(lines);

    let found = bg.all_outer_bags_for_bag("shiny gold");
    println!("{:?} -- {:?}", found.len(), found);

    let problem2 = bg.bags_within_bag("shiny gold");
    println!("Problem 2 -- {:?}", problem2);
}

pub fn all_lines(file_name: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let v: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    return v;
}
