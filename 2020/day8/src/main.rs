extern crate utils;

use std::collections::HashSet;

use utils::parse_input_to_vec;

#[derive(Debug, PartialEq)]
enum OpCode {
    Acc,
    Jmp,
    Nop,
    Nil,
}

#[derive(Debug)]
struct Instruction {
    opcode: OpCode,
    value: i32,
}

fn instruction_parser(line: &str) -> Instruction {
    let mut words_iter = line.split_whitespace();

    let opcode = match words_iter.next() {
        Some("acc") => OpCode::Acc,
        Some("jmp") => OpCode::Jmp,
        Some("nop") => OpCode::Nop,
        _ => OpCode::Nil,
    };

    let value = words_iter.next().unwrap().parse::<i32>().unwrap();

    Instruction {
        opcode,
        value,
    }
}

fn problem1(instructions: &Vec<Instruction>) {
    let mut accumulator: i32 = 0;
    let mut instruction_idx: i32 = 0;
    let mut instructions_read = HashSet::new();

    while (instruction_idx as usize) < instructions.len() {
        // println!("{} - {} - {:?}", instruction_idx+1, accumulator, instructions_read);
        if instructions_read.contains(&instruction_idx) {
            break;
        }

        let instruction = &instructions[instruction_idx as usize];
        match instruction.opcode {
            OpCode::Acc => accumulator += instruction.value,
            OpCode::Jmp => instruction_idx += instruction.value - 1,
            OpCode::Nop => (),
            _ => (),
        };

        instructions_read.insert(instruction_idx);
        instruction_idx += 1;
    }

    println!("Problem 1 -> {}", accumulator);
}

fn problem2(instructions: &Vec<Instruction>) {
    let mut accumulator: i32 = 0;
    let mut instruction_idx: i32 = 0;
    let mut instructions_read = HashSet::<i32>::new();
    let mut instructions_swaped = HashSet::<i32>::new();
    let mut swaped = false;

    while (instruction_idx as usize) < instructions.len() {
        if instructions_read.contains(&instruction_idx) {
            instruction_idx = 0;
            instructions_read = HashSet::<i32>::new();
            accumulator = 0;
            swaped = false;
        }

        instructions_read.insert(instruction_idx);

        let instruction = &instructions[instruction_idx as usize];
        match instruction.opcode {
            OpCode::Acc => accumulator += instruction.value,
            OpCode::Jmp => {
                if !swaped && instructions_swaped.insert(instruction_idx) {
                    swaped = true;
                    ()
                } else {
                    instruction_idx += instruction.value - 1
                }
            },
            OpCode::Nop => {
                if !swaped && instructions_swaped.insert(instruction_idx) {
                    swaped = true;
                    instruction_idx += instruction.value - 1
                } else {
                    ()
                }
            },
            _ => (),
        };

        instructions_read.insert(instruction_idx);
        instruction_idx += 1;
    }

    println!("Problem 2 -> {}", accumulator);
}

fn main() {
    let instructions = parse_input_to_vec::<Instruction>(
        "input",
        "\n",
        instruction_parser,
    );

    problem1(&instructions);
    problem2(&instructions);
}
