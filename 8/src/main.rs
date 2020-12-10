use aoc::common::*;
use aoc::machine::*; 

fn main() {
    problem1("input.txt");
    problem2("input.txt");
}

fn problem1(filename: &str) {
    println!("1. Find the last accumulator value before failure");
    
    let i = read_file(filename).expect("No File");
    let instructions = parse_input(&i).unwrap();
    let mut machine = Machine::new(instructions);

    match machine.run() {
        Some(MachineState::Compelte) => println!("-Instructions Accumulator result:{:?}", machine.accumulator),
        Some(MachineState::InfiniteLoop)=> println!("-Infinite Loop Detected:{:?}", machine.accumulator),
        None => panic!("-Recursion Detected e:{:?}")
    };
}

fn problem2(filename: &str) {
    println!("2. Change one of the ops to get this program to work");
    let i = read_file(filename).expect("No File");
    let instructions = parse_input(&i).unwrap();
    
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
        let mut instructions2 = instructions.clone(); 

        let item = instructions2.get_mut(index).unwrap();
        match item {
            Instruction::Nop(os) => *item = Instruction::Jmp(*os),
            Instruction::Jmp(os) => *item = Instruction::Nop(*os),
            _ => (),
        }
        let mut m = Machine::new(instructions2); 
        match m.run() {
            Some(MachineState::Compelte) => println!("\n-Instructions Accumulator : {:?} Changed : {:?}",m.accumulator, index),
            Some(MachineState::InfiniteLoop) => (),
            None => panic!("."),
        };
    }
}
