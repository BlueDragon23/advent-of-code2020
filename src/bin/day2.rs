use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use regex::{Regex};

fn main() {
    let f = File::open("input/input2_1.txt").unwrap();
    let reader = BufReader::new(f);
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();

    let result: i32 = reader.lines()
        .map(|line| line.unwrap())
        .map(|s| {
            let matches = re.captures(&s).unwrap();
            let lower = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let upper = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let c = matches.get(3).unwrap().as_str().chars().next().unwrap();
            let text = matches.get(4).unwrap().as_str();
            let text_array = text.chars().collect::<Vec<char>>();
            if (*text_array.get(lower - 1).unwrap() == c) ^ (*text_array.get(upper - 1).unwrap() == c) {
                1
            } else {
                0
            }
        }).sum();
    println!("{}", result);
}

fn validation_1(lower: usize, upper: usize, c: char, text: &str) -> i32 {
    let mut count = 0;
    for letter in text.chars() {
        if letter == c {
            count += 1;
        }
    }
    if count >= lower && count <= upper {
        1
    } else {
        0
    }
}