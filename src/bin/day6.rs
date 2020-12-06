#![feature(iterator_fold_self)]
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;

fn main() {
    let f = File::open("input/input6_1.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer);

    let result: usize = buffer.split("\n\n").map(|group| accumulate2(group.to_string())).sum();
    println!("{:?}", result);
}

fn accumulate1(group: String) -> usize {
    let set = group.chars().filter(|c| !(*c == '\n')).collect::<HashSet<char>>();
    set.len()
}

fn accumulate2(group: String) -> usize {
    group
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .fold_first(|a, b| a.intersection(&b).map(|c| *c).collect::<HashSet<char>>())
        .unwrap_or(HashSet::new())
        .len()
}