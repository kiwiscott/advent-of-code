use aoc_6::file_splitter::FileSplitter;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

fn main() {
    let c: usize = FileSplitter::new(String::from("data.txt"))
        .map(|a| any_question_answered_in_group(&a).len())
        .sum();

    println!(
        "1. Sum the number of questions answered yes per group {:?}",
        c
    );

    let c2: usize = FileSplitter::new(String::from("data.txt"))
        .map(|a| all_questions_answered_in_group(&a).len())
        .sum();

    assert_eq!(c, 6585);
    assert_eq!(c2, 3276);


    println!(
        "2. Sum the number of questions answered yes by everyone in the group {:?}",
        c2
    );
}

fn any_question_answered_in_group(data: &str) -> HashSet<char> {
    let mut v = HashSet::new();
    for c in data.chars().filter(|p| p != &' ') {
        if !v.contains(&c) {
            v.insert(c);
        }
    }
    v
}
fn all_questions_answered_in_group(data: &str) -> Vec<char> {
    let num_in_group: u8 = data.split_ascii_whitespace().count().try_into().unwrap();
    //println!("G: {:?}", num_in_group);

    let mut v = HashMap::<char, u8>::new();
    for c in data.chars().filter(|p| p != &' ') {
        if !v.contains_key(&c) {
            v.insert(c, 1);
        } else {
            if let Some(x) = v.get_mut(&c) {
                *x = *x + 1
            };
        }
    }

    let res = v
        .into_iter()
        .filter(|k| k.1 == num_in_group)
        .map(|k| k.0)
        .collect();

    //println!("{:?}", res);
    res
}
