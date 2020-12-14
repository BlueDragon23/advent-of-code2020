use core::panic;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct State<'a> {
    mask: &'a Vec<Option<bool>>,
    mem: HashMap<u64, u64>
}

#[derive(Clone, Debug)]
enum Instruction {
    Mask(Vec<Option<bool>>),
    // destination, value
    Assignment(u64, u64),
}

fn main() {
    let f = File::open("input/input14_1.txt").unwrap();
    let reader = BufReader::new(f);
    let parsed = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .collect::<Vec<_>>();
    let initial_mask = vec![];
    let result = parsed.iter().fold(State {mask: &initial_mask, mem: HashMap::new()}, |mut state, instruction| {
        match instruction {
            Instruction::Mask(m) => State { mask: m, ..state },
            Instruction::Assignment(destination, value) => {
                let bin = format!("{:036b}", destination);

                let temp = bin.chars().zip(state.mask.iter()).map(|(c, mask)| {
                    match mask {
                        Some(true) => '1',
                        Some(false) => c,
                        None => 'X'
                    }
                }).collect::<Vec<_>>();

                let iter = temp.iter().enumerate().filter(|(_, x)| **x == 'X').collect::<Vec<_>>();

                let ps = iter.iter().fold(vec![temp.clone()], |new: Vec<Vec<char>>, (i, _)| {
                    // copy state
                    let mut updated_0s = new.iter().map(|xs| {
                        let mut copy = xs.clone();
                        copy[*i] = '0';
                        copy
                    }).collect::<Vec<_>>();
                    let updated_1s = new.iter().map(|xs| {
                        let mut copy = xs.clone();
                        copy[*i] = '1';
                        copy
                    }).collect::<Vec<_>>();
                    updated_0s.extend(updated_1s.into_iter());
                    updated_0s
                });

                let new_destinations = ps
                    .iter()
                    .map(|xs| u64::from_str_radix(xs.iter().collect::<String>().as_str(), 2).unwrap())
                    .collect::<Vec<_>>();
                for new_destination in new_destinations {
                    state.mem.insert(new_destination, *value);
                }
                state
            }
        }
    });
    println!("{:?}", result.mem.values().map(|&x| x).sum::<u64>());
}

fn run_part1(parsed: Vec<Instruction>) {
    let initial_mask = vec![];
    let result = parsed.iter().fold(State {mask: &initial_mask, mem: HashMap::new()}, |mut state, instruction| {
        match instruction {
            Instruction::Mask(m) => State { mask: m, ..state },
            Instruction::Assignment(destination, value) => {
                let bin = format!("{:036b}", value);
                let new_value = u64::from_str_radix(bin.chars().zip(state.mask.iter()).map(|(c, mask)| {
                    match mask {
                        Some(true) => '1',
                        Some(false) => '0',
                        None => c
                    }
                }).collect::<String>().as_str(), 2).unwrap_or_else(|_| panic!("Failed on binary {}", bin));
                state.mem.insert(*destination, new_value);
                state
            }
        }
    });
    println!("{:?}", result.mem.values().map(|&x| x).sum::<u64>());
}

fn parse(line: &str) -> Instruction {
    let mask_regex = Regex::new(r"mask = (.+)").unwrap();
    let assignment_regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    if mask_regex.is_match(line) {
        let captures = mask_regex.captures(line).unwrap();
        Instruction::Mask(captures[1].chars().map(|c| match c {
            '1' => Some(true),
            '0' => Some(false),
            'X' => None,
            _ => panic!("banned char {}", c)
        }).collect::<Vec<_>>())
    } else {
        let captures = assignment_regex.captures(line).unwrap();
        Instruction::Assignment(captures[1].parse().unwrap(), captures[2].parse().unwrap())
    }
}