#[macro_use]
extern crate lazy_static;
use regex::RegexSet;
//use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let c = FileSplitter::new(String::from("data.txt"))
        .filter(|s| valid_passport_2(&s))
        //.for_each(|o| println!("{:?}", o));
        .count();
    println!("{:?}", c);
}

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

struct FileSplitter {
    reader: BufReader<File>,
}

impl FileSplitter {
    pub fn new(name: String) -> FileSplitter {
        let file = File::open(name).unwrap();
        let reader = BufReader::new(file);

        FileSplitter { reader }
    }
    fn process(&self, buf: Vec<u8>) -> Option<String> {
        if buf.len() != 0 {
            let buf = String::from_utf8(buf.to_vec()).unwrap();
            Some(buf)
        } else {
            None
        }
    }
}

impl Iterator for FileSplitter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut buf = Vec::new();
        let ret;
        let mut last_len = 0;

        loop {
            match self.reader.read_until(b'\n', &mut buf) {
                Ok(0) => {
                    ret = self.process(buf);
                    break;
                }
                Ok(_n) => {
                    //Trim
                    if buf.ends_with(&[10]) {
                        buf.pop();
                        if buf.ends_with(&[13]) {
                            buf.pop();
                        }
                    }

                    if buf.len() != last_len {
                        buf.push(32);
                        last_len = buf.len();
                    } else {
                        ret = self.process(buf);
                        break;
                    }
                }
                Err(_e) => {
                    ret = self.process(buf);
                    break;
                }
            };
        }

        ret
    }
}

/*
fn string_to_hash(str: String) ->HashMap::<String,String>{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<code>[a-zA-Z]{3}):(?P<value>\w*)\b").unwrap();
    }

    let mut map  = HashMap::new();

    for cap in RE.captures_iter(&str) {
        map .insert(String::from(&cap["code"]), String::from(&cap["value"]));
    }
    return map ;
}
*/
