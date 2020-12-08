#[macro_use]
extern crate lazy_static;

use aoc::common::*;

fn main() {
    println!("Hello, world!");

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