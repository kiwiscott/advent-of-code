use aoc_6::file_splitter::FileSplitter;
use std::collections::HashMap;
use std::convert::TryInto;

fn main() {
    let p1: usize = FileSplitter::new(String::from("data.txt"))
        .map(|c| {
            let mut v = std::collections::HashSet::new();
            c.chars().filter(|p| p != &' ').for_each(|a| {
                v.insert(a);
            });
            v.len()
        })
        .sum();

    println!(
        "1. Sum the number of questions answered yes per group {:?}",
        p1
    );

    let c2: usize = FileSplitter::new(String::from("data.txt"))
        //.map(|a| all_questions_answered_in_group(&a).len())
        .map(|pass| {
            let num_in_group: u8 = pass.split_ascii_whitespace().count().try_into().unwrap();
            (pass, num_in_group)
        })
        .map(|(pass, num_in_group)| {
            pass.chars()
                .filter(|p| p != &' ')
                .fold(HashMap::<char, u8>::new(), |mut acc, a| {
                    *acc.entry(a).or_insert(0) += 1;
                    acc
                })
                .iter()
                .filter(|(_, v)| **v == num_in_group)
                .count()
        })
        .sum();

    //left these in here for refactoring testing.
    assert_eq!(p1, 6585);
    assert_eq!(c2, 3276);

    println!(
        "2. Sum the number of questions answered yes by everyone in the group {:?}",
        c2
    );
}
