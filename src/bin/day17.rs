use std::{fs::File};
use std::io::prelude::*;
use std::io::BufReader;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::time;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    On,
    Off
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

fn main() {
    let start_time = time::Instant::now();
    let f = File::open("input/input17_1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut world = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .enumerate()
        .fold(HashMap::new(), |mut m, (row, line)| {
            line
                .iter()
                .enumerate()
                .for_each(|(col, &s)| {
                    m.insert(Coord {x: col as i32, y: row as i32, z: 0, w: 0}, s);
                });
            m
        });
    for _ in 0..6 {
        world = update(&world);
        // println!("Cycle {}", i);
        // println!("{:?}", world.iter()
        //     .filter(|(coord, _)| coord.z == 0)
        //     .map(|(coord, s)| format!("{:?}: {:?}", coord, s))
        //     .collect::<Vec<_>>())
    }
    println!("part 2: {}", world.values().filter(|&&s| s == State::On).count());
    let end_time = time::Instant::now();
    println!("Took {}ms", (end_time - start_time).as_millis());
}

fn update(world: &HashMap<Coord, State>) -> HashMap<Coord, State> {
    let min_x = world.keys().map(|coord| coord.x).min().unwrap();
    let max_x = world.keys().map(|coord| coord.x).max().unwrap();
    let min_y = world.keys().map(|coord| coord.y).min().unwrap();
    let max_y = world.keys().map(|coord| coord.y).max().unwrap();
    let min_z = world.keys().map(|coord| coord.z).min().unwrap();
    let max_z = world.keys().map(|coord| coord.z).max().unwrap();
    let min_w = world.keys().map(|coord| coord.w).min().unwrap();
    let max_w = world.keys().map(|coord| coord.w).max().unwrap();
    let mut new_world = world.clone();
    for x in (min_x - 1)..(max_x + 2) {
        for y in (min_y - 1)..(max_y + 2) {
            for z in (min_z - 1)..(max_z + 2) {
                for w in (min_w - 1)..(max_w + 2) {
                    let coord = Coord {x, y, z, w};
                    let current_state = *get_state(world, coord);
                    let next_state = get_new_state(world, coord);
                    if current_state == State::Off && next_state == State::Off {
                        continue
                    }
                    new_world.insert(coord, next_state);
                }
            }
        }
    }
    new_world
}

fn get_new_state(world: &HashMap<Coord, State>, coord: Coord) -> State {
    let current_state = get_state(world, coord);
    let surrounding_active = get_surrounding_active(world, coord);
    // println!("coord: {:?} has {} active neighbours", coord, surrounding_active);
    match current_state {
        State::On => {
            if surrounding_active == 2 || surrounding_active == 3 {
                State::On
            } else {
                State::Off
            }
        },
        State::Off => {
            if surrounding_active == 3 {
                State::On
            } else {
                State::Off
            }
        }
    }
}

fn get_surrounding_active(world: &HashMap<Coord, State>, coord: Coord) -> i32 {
    let mutations: Vec<i32> = vec![-1, 0, 1];
    
    let directions: Vec<(i32, i32, i32, i32)> = mutations
        .iter()
        .cartesian_product(mutations.iter())
        .cartesian_product(mutations.iter())
        .cartesian_product(mutations.iter())
        .map(|(((&x, &y), &z), &w)| (x, y, z, w))
        .filter(|(x, y, z, w)| !(*x == 0 && *y == 0 && *z == 0 && *w == 0))
        .collect();
    directions.iter().fold(0, |acc, delta| {
        let check_coord = Coord {
            x: coord.x + delta.0,
            y: coord.y + delta.1,
            z: coord.z + delta.2,
            w: coord.w + delta.3
        };
        match get_state(world, check_coord) {
            State::On => acc + 1,
            State::Off => acc
        }
    })
}

fn get_state(world: &HashMap<Coord, State>, coord: Coord) -> &State {
    world.get(&coord).unwrap_or(&State::Off)
}

fn parse(line: &str) -> Vec<State> {
    line.chars().map(|c| {
        match c {
            '.' => State::Off,
            '#' => State::On,
            _ => panic!("")
        }
    }).collect()
}