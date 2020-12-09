use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashSet;
use reformation::Reformation;

fn main() {
    let f = File::open("input/input9_1.txt").unwrap();
    let reader = BufReader::new(f);
    let numbers = reader.lines().map(|line| line.unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
    let mut lower = 0;
    let mut upper = 1;
    let target = 90433990;
    let mut sum: i64 = numbers[0] + numbers[1];
    loop {
        if sum == target {
            break
        } else if sum > target {
            sum -= numbers[lower];
            lower += 1;
        } else {
            upper += 1;
            sum += numbers[upper];
        }
    }
    let min = numbers.clone().into_iter().skip(lower).take(upper - lower + 1).min().unwrap();
    let max = numbers.clone().into_iter().skip(lower).take(upper - lower + 1).max().unwrap();
    println!("{}, {}, {}", lower, upper, min + max);
}

fn part1(numbers: Vec<i64>) {
    let mut index = 0;
    let valid_numbers = numbers.clone().into_iter().skip(25).take_while(|n| {
        // can we sum to n?
        for a in numbers[index..(index + 25)].into_iter() {
            for b in numbers[index..(index + 25)].into_iter() {
                if a == b {
                    continue;
                }
                if a + b == *n {
                    index += 1;
                    return true
                }
            }
        }
        // Update
        false
    }).collect::<Vec<i64>>();
    println!("{:?}", valid_numbers);
    println!("{}", numbers[index + 25]);
}