use aoc::common::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

fn main() {
    let preamble_len = 25;
    let numbers: Vec<i64> = read_file("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
        
}

/*
fn problem2(numbers: Vec<i64>, number_to_find: i64) -> Option<(i64, i64)> {
    let mut nums_in_play: VecDeque<i64> = VecDeque::new();
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        match sum.cmp(&number_to_find) {
            Ordering::Less => {
                nums_in_play.push_back(numbers[index]);
                sum = sum + numbers[index];
                index = index + 1;
            }
            Ordering::Greater => {
                sum = sum - nums_in_play.pop_front().unwrap();
            }
            Ordering::Equal => {
                let min = nums_in_play.iter().map(|n| *n).min().unwrap();
                let max = nums_in_play.iter().map(|n| *n).max().unwrap();
                return Some((min, max));
            }
        }
    }
    None
}

fn problem1(preamble_length: usize, numbers: Vec<i64>) -> Option<i64> {
    //What is the first number that does cannot be represented by a sum of two numbers in the previous 'preamble' numbers?

    //Build to preamble. Rust annoys me a bit here because I could just use a slice
    let mut preamble: VecDeque<i64> = VecDeque::with_capacity(preamble_length);
    numbers
        .iter()
        .take(preamble_length)
        .for_each(|n| preamble.push_back(*n));

    //println!("Preamble {:?}-{:?}",preamble.len(),preamble);

    for n in numbers.iter().skip(preamble_length) {
        let found = preamble.iter().any(|a| preamble.contains(&(n - a)));
        if !found {
            return Some(*n);
        }
        //Take off the Preamble and put on the preamble
        preamble.pop_front();
        preamble.push_back(*n);
    }
    return None;
}
*/