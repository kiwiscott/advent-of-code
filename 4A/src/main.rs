#[macro_use]
extern crate lazy_static;
use regex::RegexSet;

use aoc::common::*;

fn main() {
    let r = read_file("data.txt").unwrap();
    let v = process_double_line_entries(r)
        .iter()
        .filter(|p| valid_passport(p))
        .count();

    println!("Problem 1: {:?}", v);
}

fn valid_passport(data: &str) -> bool {
    lazy_static! {
        static ref RESET: RegexSet = RegexSet::new(&[
            r"byr:", r"iyr:", r"eyr:", r"hgt:", r"hcl:", r"ecl:", r"pid:", r"cid:"
        ])
        .unwrap();
    }

    let matches = RESET.matches(data);
    return (0..7).all(|a| matches.matched(a));
}

/*
0:byr (Birth Year) - four digits; at least 1920 and at most 2002.
1:iyr (Issue Year) - four digits; at least 2010 and at most 2020.
2:eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
3:hgt (Height) - a number followed by either cm or in:
    If cm, the number must be at least 150 and at most 193.
    If in, the number must be at least 59 and at most 76.
4:hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
5:ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
6:pid (Passport ID) - a nine-digit number, including leading zeroes.
7:cid (Country ID) - ignored, missing or not.


lazy_static! {
    static ref PASSPORT_RULES_2: RegexSet = RegexSet::new(&[
        r"byr:(19[2-9][0-9]|200[0-2])\b",
        r"iyr:(201[0-9]|2020)\b",
        r"eyr:(202[0-9]|2030)\b",
        r"hgt:((1([5-8][0-9]|[9][0-3])cm)|((59|6[0-9]|7[0-6])in))\b",
        r"hcl:#[0-9a-f]{6}\b",
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b",
        r"pid:[0-9]{9}\b"
    ])
    .unwrap();
}

fn valid_passport_2(data: &str) -> bool {
    let matches = PASSPORT_RULES_2.matches(data);
    return (0..7).all(|a| matches.matched(a));
}
*/
