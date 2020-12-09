use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input2_1.txt").unwrap();
    let reader = BufReader::new(f);
    let re = Regex::new(r"^(\d+)-(\d+) (.): (.+)$").unwrap();

    let result: usize = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|s| {
            let matches = re.captures(&s).unwrap();
            let lower = matches.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let upper = matches.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let c = matches.get(3).unwrap().as_str().chars().next().unwrap();
            let text = matches.get(4).unwrap().as_str();
            validation_2(lower, upper, c, text)
        })
        .filter(|&b| b)
        .count();
    println!("{}", result);
}

fn validation_1(lower: usize, upper: usize, c: char, text: &str) -> bool {
    let count = text.chars().filter(|&letter| letter == c).count();
    count >= lower && count <= upper
}

fn validation_2(lower: usize, upper: usize, c: char, text: &str) -> bool {
    (text.chars().nth(lower - 1).unwrap() == c) ^ (text.chars().nth(upper - 1).unwrap() == c)
}
