#[macro_use]
extern crate lazy_static;
use aoc::common::*;
use std::fmt;

fn main() {
    problem1("input.txt");
    problem2("input.txt");
}

fn problem1(filename: &str) {
    println!("1. Find the last accumulator value before failure");
    let i = read_file(filename).expect("No File");
    let instructions = parse(i);
    match run(instructions) {
        Ok(result) => println!("-Instructions Accumulator result:{:?}", result),
        Err(e) => eprint!("-Recursion Detected e:{:?}", e),
    };
    println!("");
}
fn problem2(filename: &str) {
    println!("2. Change one of the ops to get this program to work");
    let i = read_file(filename).expect("No File");
    let instructions = parse(i);

    let to_test: Vec<usize> = instructions
        .iter()
        .enumerate()
        .filter(|(_i, x)| match x {
            Instruction::Nop(_) => true,
            Instruction::Jmp(_) => true,
            _ => false,
        })
        .map(|(i, _x)| i)
        .collect();

    for index in to_test {
        let i2 = read_file(filename).expect("No File");
        let mut instructions2 = parse(i2);

        let item = instructions2.get_mut(index).unwrap();

        match item {
            Instruction::Nop(os) => *item = Instruction::Jmp(*os),
            Instruction::Jmp(os) => *item = Instruction::Nop(*os),
            _ => (),
        }
        match run(instructions2) {
            Ok(result) => println!(
                "\n-Instructions Accumulator : {:?} Changed : {:?}",
                result, index
            ),
            Err(_e) => eprint!("."),
        };
    }
}

fn run(instructions: Vec<Instruction>) -> Result<isize, RecursionError> {
    let mut accumulator: isize = 0;
    let mut next_index: usize = 0;

    let mut processed: Vec<bool> = instructions.iter().map(|_| false).collect();

    loop {
        if processed[next_index] {
            return Err(RecursionError {
                accumulator: accumulator,
                next_index: next_index,
            });
        } else {
            processed[next_index] = true;
        }

        //println!("index: {:?} accumulator{:?} instruction:{:?}",next_index, accumulator,instructions.get(next_index));

        match instructions.get(next_index) {
            Some(Instruction::Nop(_)) => next_index += 1,
            Some(Instruction::Acc(offset)) => {
                next_index += 1;
                //Ugg Rust is Annoying
                match offset {
                    OffSet::Pos(offset) => {
                        accumulator = accumulator.checked_add(*offset as isize).unwrap()
                    }
                    OffSet::Neg(offset) => {
                        accumulator = accumulator.checked_sub(*offset as isize).unwrap()
                    }
                };
            }
            Some(Instruction::Jmp(offset)) => {
                //Ugg Rust is Annoying
                match offset {
                    OffSet::Pos(offset) => next_index = next_index.checked_add(*offset).unwrap(),
                    OffSet::Neg(offset) => next_index = next_index.checked_sub(*offset).unwrap(),
                };
            }
            Some(Instruction::End) => break,
            None => (),
        };
    }

    return Ok(accumulator);
}

#[derive(Debug)]
pub struct RecursionError {
    accumulator: isize,
    next_index: usize,
}

impl std::error::Error for RecursionError {}

impl fmt::Display for RecursionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Recursion Error accumulator:{:?} next_index:{:?}",
            self.accumulator, self.next_index
        )
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(OffSet),
    Jmp(OffSet),
    Nop(OffSet),
    End,
}
#[derive(Debug, Copy, Clone)]
enum OffSet {
    Neg(usize),
    Pos(usize),
}

fn parse(desc: String) -> Vec<Instruction> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"^(\w{3}).*([\+|-])(\d*)").unwrap();
    }

    let mut ps = Vec::<Instruction>::new();

    for line in desc.lines() {
        let caps = RE.captures(line).unwrap();

        let abs = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();

        let os = match caps.get(2).unwrap().as_str() {
            "+" => OffSet::Pos(abs),
            "-" => OffSet::Neg(abs),
            _ => panic!("Offset Not Expected"),
        };

        let op = match caps.get(1).unwrap().as_str() {
            "nop" => Instruction::Nop(os),
            "acc" => Instruction::Acc(os),
            "jmp" => Instruction::Jmp(os),
            _ => panic!("Type Not Expected"),
        };
        ps.push(op);
    }
    ps.push(Instruction::End);

    ps
}
