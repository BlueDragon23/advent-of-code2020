use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input5_1.txt").unwrap();
    let reader = BufReader::new(f);
    let seat_ids = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let row = line
                .chars()
                .into_iter()
                .take(7)
                .fold((0f32, 127f32), |(lower, upper), c| {
                    if c == 'F' {
                        (lower, lower + ((upper - lower) / 2f32).floor())
                    } else {
                        (lower + ((upper - lower) / 2f32).ceil(), upper)
                    }
                });
            let col = line
                .chars()
                .into_iter()
                .skip(7)
                .fold((0f32, 7f32), |(lower, upper), c| {
                    if c == 'L' {
                        (lower, lower + ((upper - lower) / 2f32).floor())
                    } else {
                        (lower + ((upper - lower) / 2f32).ceil(), upper)
                    }
                });
            (row.0 as i32) * 8 + (col.0 as i32)
        })
        .collect::<Vec<i32>>();
    println!("part 1: {}", seat_ids.clone().into_iter().max().unwrap());
    let mut sorted = seat_ids.clone();
    sorted.sort();
    println!(
        "part 2: {:?}",
        sorted
            .clone()
            .into_iter()
            .zip(sorted.into_iter().skip(1))
            .take_while(|(a, b)| a + 1 == *b)
            .map(|(_a, b)| b)
            .collect::<Vec<i32>>()
    );
}
