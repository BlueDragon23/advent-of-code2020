use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Floor,
    Empty,
    Occupied,
}

fn main() {
    let f = File::open("input/input11_1.txt").unwrap();
    let reader = BufReader::new(f);
    let parsed = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .collect::<Vec<_>>();
    let mut board = parsed.clone();
    let final_state_1 = loop {
        let new_board = transform(&board);
        if new_board == board {
            break board;
        }
        board = new_board;
    };
    println!("part 1: {}", count_results(&final_state_1));
    let mut board = parsed;
    let final_state = loop {
        let new_board = transform2(&board);
        if new_board == board {
            break board;
        }
        board = new_board;
    };
    println!("part 2: {}", count_results(&final_state));
}

fn parse(line: &str) -> Vec<State> {
    line.chars()
        .map(|c| match c {
            'L' => State::Empty,
            '.' => State::Floor,
            '#' => State::Occupied,
            x => panic!("Invalid state {}", x),
        })
        .collect()
}

fn count_results(board: &Vec<Vec<State>>) -> usize {
    board
        .iter()
        .map(|row| row.iter().filter(|&&x| x == State::Occupied).count())
        .sum::<usize>()
}

fn transform2(board: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    board
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, &col)| match col {
                    State::Empty => {
                        if check_surrounds_empty2(board, r, c) {
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        if check_surrounds_full2(board, r, c) {
                            State::Empty
                        } else {
                            State::Occupied
                        }
                    }
                    state => state,
                })
                .collect()
        })
        .collect()
}

fn find_next_seat(
    board: &Vec<Vec<State>>,
    r: usize,
    c: usize,
    direction: (i32, i32),
) -> Option<State> {
    let mut position = (r as i32 + direction.0, c as i32 + direction.1);
    while position.0 >= 0
        && position.1 >= 0
        && (position.0 as usize) < board.len()
        && (position.1 as usize) < board[0].len()
    {
        let state = board[position.0 as usize][position.1 as usize];
        if state != State::Floor {
            return Some(state);
        }
        position = (position.0 + direction.0, position.1 + direction.1);
    }
    Option::None
}

fn check_surrounds_empty2(board: &Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    directions.iter().all(|&direction| {
        find_next_seat(board, r, c, direction)
            .map(|state| state != State::Occupied)
            .unwrap_or(true)
    })
}

fn check_surrounds_full2(board: &Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    directions.iter().fold(0, |acc, &direction| {
        acc + find_next_seat(board, r, c, direction)
            .map(|state| if state == State::Occupied { 1 } else { 0 })
            .unwrap_or(0)
    }) >= 5
}

fn transform(board: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    board
        .iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .map(|(c, &col)| match col {
                    State::Empty => {
                        if check_surrounds_empty(board, r, c) {
                            State::Occupied
                        } else {
                            State::Empty
                        }
                    }
                    State::Occupied => {
                        if check_surrounds_full(board, r, c) {
                            State::Empty
                        } else {
                            State::Occupied
                        }
                    }
                    state => state,
                })
                .collect()
        })
        .collect()
}

fn check_surrounds_empty(board: &Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    directions.iter().all(|direction| {
        let (r_, c_) = (r as i32 + direction.0, c as i32 + direction.1);
        if r_ < 0 || r_ >= board.len() as i32 || c_ < 0 || c_ >= board[0].len() as i32 {
            return true;
        }
        board[r_ as usize][c_ as usize] != State::Occupied
    })
}

fn check_surrounds_full(board: &Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    directions.iter().fold(0, |acc, &direction| {
        let (r_, c_) = (r as i32 + direction.0, c as i32 + direction.1);
        if r_ < 0 || r_ >= board.len() as i32 || c_ < 0 || c_ >= board[0].len() as i32 {
            acc
        } else if board[r_ as usize][c_ as usize] == State::Occupied {
            acc + 1
        } else {
            acc
        }
    }) >= 4
}
