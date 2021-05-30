use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input3_1.txt").unwrap();
    let reader = BufReader::new(f);

    let lines = reader
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut results = vec![];
    for pattern in vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let mut position = (0, 0);
        let result: usize = lines
            .clone()
            .into_iter()
            .step_by(pattern.0)
            .map(|line| {
                let obstacle = line.chars().cycle().nth(position.1).unwrap();
                position = (position.0 + pattern.0, position.1 + pattern.1);
                match obstacle {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Invalid character {}", obstacle),
                }
            })
            .sum();
        results.push(result);
        println!("{}", result);
    }
    println!(
        "{}",
        results
            .into_iter()
            .fold(1, |product, value| product * value)
    );
}
