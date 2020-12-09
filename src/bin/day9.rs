use itertools::{Combinations, Itertools};
use reformation::Reformation;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input9_1.txt").unwrap();
    let reader = BufReader::new(f);
    let numbers = reader
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    part1(&numbers);
    part2(&numbers);
}

fn part1(numbers: &Vec<i64>) {
    let result = numbers.windows(26).find(|window| {
        let target = *window.last().unwrap();
        window[0..25]
            .into_iter()
            .combinations(2)
            .any(|xs| xs.into_iter().map(|x| *x).sum::<i64>() == target)
    });
    println!("{:?}", result.unwrap().last());
}

fn part2(numbers: &Vec<i64>) {
    let mut lower = 0;
    let mut upper = 1;
    let target = 90433990;
    let mut sum: i64 = numbers[0] + numbers[1];
    loop {
        if sum == target {
            break;
        } else if sum > target {
            sum -= numbers[lower];
            lower += 1;
        } else {
            upper += 1;
            sum += numbers[upper];
        }
    }
    let min = numbers
        .clone()
        .into_iter()
        .skip(lower)
        .take(upper - lower + 1)
        .min()
        .unwrap();
    let max = numbers
        .clone()
        .into_iter()
        .skip(lower)
        .take(upper - lower + 1)
        .max()
        .unwrap();
    println!("{}, {}, {}", lower, upper, min + max);
}
