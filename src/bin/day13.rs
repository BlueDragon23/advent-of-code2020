use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy)]
enum Id {
    Id(i32),
    X,
}

fn main() {
    let f = File::open("input/input13_1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut iter = reader.lines();
    let timestamp = iter.next().unwrap().unwrap().parse::<i32>().unwrap();
    let ids = iter.next().unwrap().unwrap().split(",").map(|x| {
        if x == "x" {
            Id::X
        } else {
            Id::Id(x.parse::<i32>().unwrap())
        }
    }).collect::<Vec<_>>();
    part1(timestamp, ids.clone());
    part2(ids);
}

fn part1(timestamp: i32, ids: Vec<Id>) {
    let running_buses = ids.into_iter().filter(|id| match id {
        Id::Id(_) => true,
        _ => false
    }).collect::<Vec<_>>();
    let closest = running_buses.into_iter().map(|id| {
        match id {
            Id::Id(x) => (x, x - (timestamp % x)),
            _ => panic!("Filtered out")
        }
    }).min_by_key(|t| t.1);
    println!("{:?}", closest);
}

fn part2(ids: Vec<Id>) {
    let mut timestamp: u128 = 100_000_000_000_000;
    let running_buses = ids.into_iter().enumerate().filter(|(_, id)| match id {
        Id::Id(_) => true,
        _ => false
    }).collect::<Vec<_>>();
    loop {
        if running_buses.iter().all(|&(index, x)| {
            let result = match x {
                Id::X => true,
                Id::Id(value) => (timestamp + (index as u128)) % (value as u128) == 0
            };
            result
        }) {
            break
        }
        timestamp += 1;
    }
    println!("{}", timestamp);
}