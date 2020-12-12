use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use reformation::Reformation;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct State {
    direction: i32,
    x: i32,
    y: i32,
    waypoint_x: i32,
    waypoint_y: i32
}

#[derive(Reformation)]
enum Instruction {
    #[reformation(r"N{}")]
    North(i32),
    #[reformation(r"S{}")]
    South(i32),
    #[reformation(r"E{}")]
    East(i32),
    #[reformation(r"W{}")]
    West(i32),
    #[reformation(r"F{}")]
    Forward(i32),
    #[reformation(r"L{}")]
    Left(i32),
    #[reformation(r"R{}")]
    Right(i32)
}

fn main() {
    let f = File::open("input/input12_1.txt").unwrap();
    let reader = BufReader::new(f);
    let end_state = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .fold(State {direction: 0, x: 0, y: 0, waypoint_x: 10, waypoint_y: 1}, |state, instruction| {
            match instruction {
                Instruction::North(d) => State { waypoint_y: state.waypoint_y + d, .. state },
                Instruction::South(d) => State { waypoint_y: state.waypoint_y - d, .. state },
                Instruction::East(d) => State { waypoint_x: state.waypoint_x + d, .. state },
                Instruction::West(d) => State { waypoint_x: state.waypoint_x - d, .. state },
                Instruction::Forward(d) => State { x: state.x + state.waypoint_x * d, y: state.y + state.waypoint_y * d, .. state },
                Instruction::Left(d) => match d {
                        0 => State { waypoint_x: state.waypoint_x, waypoint_y: state.waypoint_y, .. state },
                        90 => State { waypoint_x: -state.waypoint_y, waypoint_y: state.waypoint_x, .. state },
                        180 => State { waypoint_x: -state.waypoint_x, waypoint_y: -state.waypoint_y, .. state },
                        270 => State { waypoint_x: state.waypoint_y, waypoint_y: -state.waypoint_x, .. state },
                        _ => panic!("Invalid distance {}", d)
                    },
                Instruction::Right(d) => match d {
                    0 => State { waypoint_x: state.waypoint_x, waypoint_y: state.waypoint_y, .. state },
                    270 => State { waypoint_x: -state.waypoint_y, waypoint_y: state.waypoint_x, .. state },
                    180 => State { waypoint_x: -state.waypoint_x, waypoint_y: -state.waypoint_y, .. state },
                    90 => State { waypoint_x: state.waypoint_y, waypoint_y: -state.waypoint_x, .. state },
                    _ => panic!("Invalid distance {}", d)
                },
            }
    });
    println!("{:?}", end_state);
}

fn update_part1(state: State, instruction: Instruction) -> State {
    match instruction {
        Instruction::North(d) => State { y: state.y + d, .. state },
        Instruction::South(d) => State { y: state.y - d, .. state },
        Instruction::East(d) => State { x: state.x + d, .. state },
        Instruction::West(d) => State { x: state.x - d, .. state },
        Instruction::Forward(d) => {
            if state.direction == 0 {
                State { x: state.x + d, .. state }
            } else if state.direction == 90 {
                State { y: state.y + d, .. state }
            } else if state.direction == 180 {
                State { x: state.x - d, .. state }
            } else if state.direction == 270 {
                State { y: state.y - d, .. state }
            } else {
                panic!("Invalid direction {}", state.direction);
            }
        },
        Instruction::Left(d) => State { direction: (state.direction + d) % 360, .. state },
        Instruction::Right(d) => State { direction: (state.direction - d + 360) % 360, .. state },
    }
}

fn parse(line: &str) -> Instruction {
    Instruction::parse(line).unwrap()
}