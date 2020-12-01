use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input/input1_1.txt").unwrap();
    let reader = BufReader::new(f);
    let numbers = reader.lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    for a in &numbers {
        for b in &numbers {
            for c in &numbers {
                if *a + *b + *c == 2020 {
                    println!("a * b * c = {}", (*a * *b * *c));
                }
            }
        } 
    }
}