use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Floor,
    Seat,
    Occupied
}

fn main() {
    let f = File::open("input/input11_1.txt").unwrap();
    let reader = BufReader::new(f);
    let parsed = reader
        .lines()
        .map(|line| parse(line.unwrap().as_str()))
        .collect::<Vec<_>>();
    let mut board = parsed;
    let mut iterations = 0;
    let final_state = loop {
        let new_board = transform2(board.clone());
        if new_board == board {
            break board;
        }
        board = new_board;
        // println!("{:?}", board);
        // println!("{}", board.iter().fold("".to_string(), |acc, row| acc + row.iter().fold("", |a, col| {
        //     a.to_string().push(match col {
        //     State::Seat => 'L',
        //     State::Floor => '.',
        //     State::Occupied => '#'
        // }); a}) + "\n"));
        iterations += 1;
        if iterations % 10 == 0 {
            println!("{}", iterations);
        }
    };
    println!("{}", final_state.iter().map(|row| row.iter().filter(|&&x| x == State::Occupied).count()).sum::<usize>());
}

fn parse(line: &str) -> Vec<State> {
    line.chars().map(|c| match c {
        'L' => State::Seat,
        '.' => State::Floor,
        '#' => State::Occupied,
        x => panic!("Invalid state {}", x)
    }).collect()
}

fn transform2(board: Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut new_board = board.clone();
    for (r, row) in board.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            let new_state = match col {
                State::Seat => if check_surrounds_empty2(board.clone(), r, c) {
                    State::Occupied
                } else {
                    State::Seat
                },
                State::Occupied => if check_surrounds_full2(board.clone(), r, c) {
                    State::Seat
                } else {
                    State::Occupied
                }
                state => state
            };
            new_board[r][c] = new_state;
        }
    }
    new_board
}

fn find_next_seat(board: Vec<Vec<State>>, r: usize, c: usize, direction: (i32, i32)) -> Option<State> {
    let mut position = (r as i32 + direction.0, c as i32 + direction.1);
    while position.0 >=0 && position.1 >= 0 && (position.0 as usize) < board.len() && (position.1 as usize) < board[0].len() {
        let state = board[position.0 as usize][position.1 as usize];
        if state != State::Floor {
            return Some(state);
        }
        position = (position.0 + direction.0, position.1 + direction.1);
    }
    Option::None
}

fn check_surrounds_empty2(board: Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    for direction in directions {
        let option_next = find_next_seat(board.clone(), r, c, direction);
        if option_next.is_none() {
            continue;
        }
        if option_next.unwrap() == State::Occupied {
            return false
        }
    }
    true
}

fn check_surrounds_full2(board: Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut count = 0;
    for direction in directions {
        let option_next = find_next_seat(board.clone(), r, c, direction);
        if option_next.is_none() {
            continue;
        }
        if option_next.unwrap() == State::Occupied {
            // println!("{}, {}", r, c);
            count += 1;
        }
    }
    // println!("{}, {}, {}", r, c, count);
    count >= 5
}

fn transform(board: Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut new_board = board.clone();
    for (r, row) in board.iter().enumerate() {
        for (c, &col) in row.iter().enumerate() {
            let new_state = match col {
                State::Seat => if check_surrounds_empty(board.clone(), r, c) {
                    State::Occupied
                } else {
                    State::Seat
                },
                State::Occupied => if check_surrounds_full(board.clone(), r, c) {
                    State::Seat
                } else {
                    State::Occupied
                }
                state => state
            };
            new_board[r][c] = new_state;
        }
    }
    new_board
}

fn check_surrounds_empty(board: Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    for direction in directions {
        let (r_, c_) = (r as i32 + direction.0, c as i32 + direction.1);
        if r_ < 0 || r_ >= board.len() as i32 || c_ < 0 || c_ >= board[0].len() as i32 {
            continue
        }
        if board[r_ as usize][c_ as usize] == State::Occupied {
            return false
        }
    }
    true
}

fn check_surrounds_full(board: Vec<Vec<State>>, r: usize, c: usize) -> bool {
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];
    let mut count = 0;
    for direction in directions {
        let (r_, c_) = (r as i32 + direction.0, c as i32 + direction.1);
        if r_ < 0 || r_ >= board.len() as i32 || c_ < 0 || c_ >= board[0].len() as i32 {
            continue
        }
        if board[r_ as usize][c_ as usize] == State::Occupied {
            // println!("{}, {}", r, c);
            count += 1;
        }
    }
    // println!("{}, {}, {}", r, c, count);
    count >= 4
}