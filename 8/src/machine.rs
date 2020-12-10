use std::collections::HashSet;

pub type Instructions = Vec<Instruction>;

#[derive(Debug, Copy, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
    End,
}

#[derive(Debug, Copy, Clone)]
pub enum MachineState {
    Compelte,
    InfiniteLoop,
}

pub struct Machine {
    pub accumulator: i64,
    pub instructions: Instructions,
}

impl Machine {
    pub fn new(instructions: Instructions) -> Self {
        Machine {
            accumulator: 0,
            instructions: instructions,
        }
    }
    pub fn reset(&mut self,instructions: Instructions){
        self.accumulator = 0; 
        self.instructions = instructions
    }

    pub fn run(&mut self) -> Option<MachineState> {
        let mut next_index: i64 = 0;
        let mut processed = HashSet::<i64>::new();

        loop {
            if !processed.insert(next_index) {
                return Some(MachineState::InfiniteLoop);
            }

            match self.instructions.get(next_index as usize) {
                Some(Instruction::Nop(_)) => next_index += 1,
                Some(Instruction::Acc(offset)) => {
                    next_index += 1;
                    self.accumulator += offset;
                }
                Some(Instruction::Jmp(offset)) => {
                    next_index += offset;
                }
                Some(Instruction::End) => break,
                None => (),
            };
        }

        return Some(MachineState::Compelte);
    }
}


pub fn parse_input(input: &str) -> Option<Instructions> {
    lazy_static! {
        static ref RE: regex::Regex = regex::Regex::new(r"^(\w{3}).*([\+|-]\d*)").unwrap();
    }

    let mut ps = Instructions::new();
    for line in input.lines() {
        let caps = RE.captures(line).unwrap();
        let os = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let op = match caps.get(1).unwrap().as_str() {
            "nop" => Instruction::Nop(os),
            "acc" => Instruction::Acc(os),
            "jmp" => Instruction::Jmp(os),
            _ => panic!("Type Not Expected"),
        };
        ps.push(op);
    }
    ps.push(Instruction::End);
    Some(ps)
}

