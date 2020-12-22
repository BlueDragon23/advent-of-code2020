#![feature(str_split_once)]
#![feature(iter_map_while)]
use std::{collections::{HashMap, HashSet}, fs::File};
use std::io::prelude::*;
use std::io::BufReader;
use log::LevelFilter;
use regex::Regex;
use simple_logger::SimpleLogger;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum RuleType {
    Terminal(char),
    Alternation(Sequence, Sequence),
    Sequence(Sequence)
}

#[derive(Copy, Clone, Debug)]
struct Sequence {
    first: usize,
    second: Option<usize>,
    third: Option<usize>
}

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Error).init().unwrap();
    // Different input for part 1 + part 2
    let f = File::open("input/input19_1.txt").unwrap();
    let reader = BufReader::new(f);
    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<_>>();
    let rules = lines.iter()
        .map_while(|line| if line.is_empty() {
            None
        } else {
            Some(parse_rule(line))
        })
        .collect::<HashMap<_, _>>();
    let simplified_rules = rules
        .keys()
        .map(|rule| (rule, simplify_rule(&rules, *rule)))
        .collect::<HashMap<_, _>>();

    let messages = lines.iter()
        .skip(rules.len() + 1)
        .collect::<Vec<_>>();
    // Check using simplified rules. O(n^2) lol
    let mut sum = 0;
    let forty_two = format!("({})", simplified_rules[&42].iter().join("|"));
    let thirty_one = format!("({})", simplified_rules[&31].iter().join("|"));

    for message in &messages {
        if valid_simplified(&forty_two, &thirty_one, message) {
            sum += 1;
        }
    }
    println!("Part 2: {}", sum);
    
    let valid_messages = messages
        .iter()
        .filter(|&&m| {
            let (valid, final_index) = is_valid(m, &rules, 0, 0);
            log::debug!("message: {}, len: {}, valid: {}, final_index: {}", m, m.len(), valid, final_index);
            valid && final_index == m.len()
        })
        .count();
    println!("Part 1: {}", valid_messages);
}

fn valid_simplified(forty_two: &String, thirty_one: &String, line: &str) -> bool {
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    // 0: 8 11
    let regex = Regex::new(format!("^(?P<left>{}{}+)(?P<right>{}+)$", forty_two, forty_two, thirty_one).as_str()).unwrap();
    if let Some(captures) = regex.captures(line) {
        let left_regex = Regex::new(forty_two).unwrap();
        let right_regex = Regex::new(thirty_one).unwrap();
        // Verify length
        let left_count = left_regex.find_iter(&captures["left"]).count();
        let right_count = right_regex.find_iter(&captures["right"]).count();
        return left_count > right_count
    }
    false
}

fn simplify_rule(rules: &HashMap<usize, RuleType>, rule_id: usize) -> HashSet<String> {
    let rule = rules[&rule_id];
    match rule {
        RuleType::Terminal(c) => {
            let mut s = HashSet::new();
            s.insert(c.to_string());
            s
        },
        RuleType::Sequence(sequence) => {
            simplify_sequence(rules, sequence)
        },
        RuleType::Alternation(first, second) => {
            vec![simplify_sequence(rules, first), simplify_sequence(rules, second)]
                .into_iter()
                .concat()
        }
    }
}

fn simplify_sequence(rules: &HashMap<usize, RuleType>, sequence: Sequence) -> HashSet<String> {
    let mut result = simplify_rule(rules, sequence.first);
    if let Some(two) = sequence.second {
        let second = simplify_rule(rules, two);
        result = result
            .into_iter()
            .cartesian_product(second.iter().collect::<Vec<_>>())
            .map(|(mut a, b)| {
                a.extend(b.chars());
                a
            })
            .collect();
        if let Some(three) = sequence.third {
            let third = simplify_rule(rules, three);
            result = result
                .into_iter()
                .cartesian_product(third.iter().collect::<Vec<_>>())
                .map(|(mut a, b)| {
                    a.extend(b.chars());
                    a
                })
                .collect();
        }
    }
    result
}

fn is_valid(message: &String, rules: &HashMap<usize, RuleType>, rule_id: usize, index: usize) -> (bool, usize) {
    if index >= message.len() {
        return (false, index)
    }
    log::debug!("Applying rule {}", rule_id);
    match rules[&rule_id] {
        RuleType::Terminal(c) => {
            log::debug!("Checking index {} against terminal {}", index, c);
            // If we're on a terminal rule, check that the index we're up to matches
            (message.chars().nth(index).unwrap() == c, index + 1)
        },
        RuleType::Alternation(left, right) => {
            log::debug!("Checking index {} against left alternation {:?}", index, left);
            // If we're on an alternation, check the left side then the right
            let (mut result, mut new_index) = is_valid_sequence(message, rules, index, left.first, left.second, left.third);
            if !result {
                log::debug!("Checking index {} against right alternation {:?}", index, right);
                let (new_result, right_index) = is_valid_sequence(message, rules, index, right.first, right.second, right.third);
                result |= new_result;
                new_index = right_index;
            }
            (result, new_index)
        },
        RuleType::Sequence(sequence) => {
            log::debug!("Checking index {} against sequence {:?}", index, sequence);
            // If we're on a sequence, check the sequence
            is_valid_sequence(message, rules, index, sequence.first, sequence.second, sequence.third)
        }
    }
}

fn is_valid_sequence(message: &String, rules: &HashMap<usize, RuleType>, index: usize, 
    first: usize, second: Option<usize>, third: Option<usize>) -> (bool, usize) {
        let (mut result, mut new_index) = is_valid(message, rules, first, index);
        if result && second.is_some() {
            // Only check second part of the sequence if the first one succeeded
            let (new_result, second_index) = is_valid(message, rules, second.unwrap(), new_index);
            result &= new_result;
            new_index = second_index;
        }
        if result && third.is_some() {
            // Only check second part of the sequence if the first one succeeded
            let (new_result, third_index) = is_valid(message, rules, third.unwrap(), new_index);
            result &= new_result;
            new_index = third_index;
        }
        (result, new_index)
}

fn parse_rule(line: &str) -> (usize, RuleType) {
    let terminal_pattern = Regex::new("\"(.)\"").unwrap();
    let alternation_pattern = Regex::new(r"(?P<first>\d+)( (?P<second>\d+)( (?P<third>\d+))?)? \| (?P<fourth>\d+)( (?P<fifth>\d+)( (?P<sixth>\d+))?)?").unwrap();
    let sequence_pattern = Regex::new(r"(?P<first>\d+)( (?P<second>\d+)( (?P<third>\d+))?)?").unwrap();
    let (id_str, contents) = line
        .split_once(": ")
        .unwrap();
    let id = id_str.parse::<usize>().unwrap();
    if let Some(terminal) = terminal_pattern.captures(contents) {
        return  (
            id,
            RuleType::Terminal(terminal[1].chars().next().unwrap())
        )
    }
    if let Some(alternation) = alternation_pattern.captures(contents) {
        return  (
            id,
            RuleType::Alternation(
                Sequence {
                    first: alternation.name("first").unwrap().as_str().parse().unwrap(),
                    second: alternation.name("second").map(|x| x.as_str().parse::<usize>().unwrap()),
                    third: alternation.name("third").map(|x| x.as_str().parse::<usize>().unwrap()),
                },
                Sequence {
                    first: alternation.name("fourth").unwrap().as_str().parse().unwrap(),
                    second: alternation.name("fifth").map(|x| x.as_str().parse::<usize>().unwrap()),
                    third: alternation.name("sixth").map(|x| x.as_str().parse::<usize>().unwrap()),
                },
            )
        )
    }
    if let Some(sequence) = sequence_pattern.captures(contents) {
        return  (
            id,
            RuleType::Sequence(
                Sequence {
                    first: sequence.name("first").unwrap().as_str().parse().unwrap(),
                    second: sequence.name("second").map(|x| x.as_str().parse::<usize>().unwrap()),
                    third: sequence.name("third").map(|x| x.as_str().parse::<usize>().unwrap()),
                },
            )
        )
    }
    panic!("Failed to match any patterns in {}", line);
}