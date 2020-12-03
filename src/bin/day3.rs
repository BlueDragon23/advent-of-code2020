use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input3_1.txt").unwrap();
    let reader = BufReader::new(f);

    let tree = '#';
    let empty = '.';
    let right_movement = 3;
    let down_movement = 1;
    let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
    for right_movement in vec![1, 3, 5, 7] {
        let mut position = (0, 0);
        let result: usize = lines.clone().into_iter().map(|line| {
            let obstacle = line.chars().cycle().nth(position.1).unwrap();
            position = (position.0 + down_movement, position.1 + right_movement);
            if obstacle == tree {
                1
            } else {
                0
            }
        }).sum();
        println!("{}", result);
    }
    let mut position = (0, 0);
    let result: usize = lines.clone().into_iter().step_by(2).map(|line| {
        let obstacle = line.chars().cycle().nth(position.1).unwrap();
        position = (position.0 + 2, position.1 + 1);
        if obstacle == tree {
            1
        } else {
            0
        }
    }).sum();
    println!("{}", result);
}