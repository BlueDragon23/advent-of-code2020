use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input6_1.txt").unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();
    reader
        .read_to_string(&mut buffer)
        .unwrap_or_else(|_| panic!("Failed to read"));

    let part1: usize = buffer
        .split("\n\n")
        .map(|group| accumulate1(group.to_string()))
        .sum();
    println!("part 1: {:?}", part1);

    let part2: usize = buffer
        .split("\n\n")
        .map(|group| accumulate2(group.to_string()))
        .sum();
    println!("part 2: {:?}", part2);
}

fn accumulate1(group: String) -> usize {
    let set = group
        .chars()
        .filter(|c| !(*c == '\n'))
        .collect::<HashSet<char>>();
    set.len()
}

fn accumulate2(group: String) -> usize {
    group
        .lines()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).map(|c| *c).collect::<HashSet<char>>())
        .unwrap_or(HashSet::new())
        .len()
}
