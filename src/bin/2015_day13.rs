use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Edge {
    source: String,
    target: String,
    weight: i32
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Key {
    source: String,
    target: String,
}

fn main() {
    let f = File::open("input/input13_1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut parsed = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .fold(HashMap::new(), |mut m, x| {
            m.insert(Key {source: x.source, target: x.target}, x.weight);
            m
        });
    let mut people = parsed.iter().map(|e| e.0.source.clone()).unique().collect::<Vec<String>>();
    people.iter().for_each(|person| {
        parsed.insert(Key { source: "me".to_string(), target: person.clone() }, 0);
        parsed.insert(Key { source: person.clone(), target: "me".to_string() }, 0);
    });
    people.push("me".to_string());
    let result = people
        .iter()
        .permutations(people.len())
        .map(|combination| {
            // find weight
            let happiness = combination
                .iter()
                .cycle()
                .tuple_windows()
                .take(people.len())
                .fold(0, |total_happiness, (&left, &person, &right)| {
                    total_happiness + 
                        parsed.get(&Key { source: person.clone(), target: left.clone() }).unwrap() + 
                        parsed.get(&Key { source: person.clone(), target: right.clone() }).unwrap()
                });
            (happiness, combination)
        }).max_by_key(|x| x.0).unwrap();
    println!("{}", result.0);
}

fn parse(line: &str) -> Edge {
    let re = Regex::new(r"(.+) would (lose|gain) (\d+) happiness units by sitting next to (.+).").unwrap();
    let captures = re.captures(line).unwrap_or_else(|| panic!("Failed on line {}", line));
    let source = captures[1].to_string();
    let mut value = captures[3].parse::<i32>().unwrap();
    if captures[2] == *"lose" {
        value = -value;
    }
    let target = captures[4].to_string();
    Edge { source, target, weight: value }
}