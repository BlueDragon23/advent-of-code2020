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

#[derive(Reformation)]
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
        let new_program = match Instruction::parse(program[change_line].as_str()).unwrap() {
            Instruction::Acc(_) => continue,
            Instruction::Jump(x) => {
                let mut new = program.clone();
                new[change_line] = format!("nop {}", x);
                new
            }
            Instruction::Noop(x) => {
                let mut new = program.clone();
                new[change_line] = format!("jmp {}", x);
                new
            }
        };
        let mut executed_lines = HashSet::new();
        let mut state = State { line: 0, acc: 0 };
        loop {
            if executed_lines.contains(&state.line) || state.line >= program.len() {
                break;
            }
            executed_lines.insert(state.line);
            let function = Instruction::parse(new_program[state.line].as_str()).unwrap();
            match function {
                Instruction::Jump(x) => state.line = ((state.line as i32) + x) as usize,
                Instruction::Acc(x) => {
                    state.acc += x;
                    state.line += 1
                }
                _ => state.line += 1,
            }
        }
        if state.line == program.len() {
            println!("{:?}", state.acc);
        }
    }
}
