use reformation::Reformation;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct State {
    line: usize,
    acc: i32,
}

#[derive(Clone, Copy, Reformation)]
enum Instruction {
    #[reformation(r"acc {}")]
    Acc(i32),
    #[reformation(r"jmp {}")]
    Jump(i32),
    #[reformation(r"nop {}")]
    Noop(i32),
}

fn main() {
    let f = File::open("input/input8_1.txt").unwrap();
    let reader = BufReader::new(f);
    let program = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    for change_line in 0..program.len() {
        let replacement = match get_replacement_instruction(program[change_line].as_str()) {
            Some(instr) => instr,
            _ => continue,
        };
        let state = run_program(program.clone(), change_line, replacement);
        if state.line == program.len() {
            println!("{:?}", state.acc);
            break;
        }
    }
}

fn get_replacement_instruction(line: &str) -> Option<Instruction> {
    match Instruction::parse(line).unwrap() {
        Instruction::Acc(_) => Option::None,
        Instruction::Jump(x) => {
            Some(Instruction::Noop(x))
        }
        Instruction::Noop(x) => {
            Some(Instruction::Jump(x))
        }
    }
}

fn run_program(program: Vec<String>, change_line: usize, replacement: Instruction) -> State {
    let mut executed_lines = HashSet::new();
    let mut state = State { line: 0, acc: 0 };
    loop {
        if executed_lines.contains(&state.line) || state.line >= program.len() {
            break;
        }
        executed_lines.insert(state.line);
        let function = if state.line == change_line {
            replacement
        } else {
            Instruction::parse(program[state.line].as_str()).unwrap()
        };
        match function {
            Instruction::Jump(x) => state.line = ((state.line as i32) + x) as usize,
            Instruction::Acc(x) => {
                state.acc += x;
                state.line += 1
            }
            _ => state.line += 1,
        }
    }
    state
}
