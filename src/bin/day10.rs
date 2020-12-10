use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let f = File::open("input/input10_1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut parsed = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .collect::<Vec<_>>();
    // Start at 0
    parsed.insert(0, 0);
    parsed.sort();
    // Get combinations
    println!("{}", part1(parsed.clone()));
    println!("{:?}", part2_recursive(parsed, 0, &mut HashMap::new()).0);
}

fn parse(line: &str) -> u128 {
    line.parse::<_>().unwrap()
}

fn part1(parsed: Vec<u128>) -> usize {
    let mut m = 0;
    let mut one_count = 0;
    let mut three_count = 1;
    for n in parsed {
        if n - m == 1 {
            one_count += 1;
        } else if n - m == 3 {
            three_count += 1;
        }
        m = n;
    }
    one_count * three_count
}

fn part2_recursive(parsed: Vec<u128>, index: usize, cache: &mut HashMap<usize, u128>) -> (u128, &HashMap<usize, u128>) {
    let maybe_result = cache.get(&index);
    if maybe_result.is_some() {
        return (*maybe_result.unwrap(), cache);
    }
    let number = parsed[index];
    // Get possibilities
    // DFS
    let result = parsed
        .iter()
        .enumerate()
        .skip(index + 1)
        .take(3)
        .filter(|(_, &x)| x - number <= 3)
        .map(|(i, _)| part2_recursive(parsed.clone(), i, cache).0)
        .collect::<Vec<_>>();
    if result.len() == 0 {
        cache.insert(index, 1);
        return (1, cache);
    }
    let result_sum: u128 = result.iter().sum();
    cache.insert(index, result_sum);
    (result_sum, cache)
}