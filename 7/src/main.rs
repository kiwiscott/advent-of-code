use aoc::bag_map::BagMap;

fn main() {
    let lines = all_lines("input.txt");
    let bg = BagMap::new(lines);

    let found = bg.all_outer_bags_for_bag("shiny gold");
    assert_eq!(131,found.iter().count(), "Expected 131. Found {:?}",found.iter().count());
    println!("Problem 1 -- {:?}", found.iter().count());


    let problem2 = bg.bags_within_bag("shiny gold");
    
    //Calculation will include this bag which we don't need 
    static OUTER_BAG: u32 = 1;
    println!("Problem 2 -- {:?}", problem2 - OUTER_BAG);
}

pub fn all_lines(file_name: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let v: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    return v;
}
