use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::io::prelude::*;
use std::io::BufReader;
use std::{
    collections::{VecDeque},
    fs::File,
};

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .init()
        .unwrap();
    let f = File::open("input/input22_1.txt").unwrap();
    let reader = BufReader::new(f);
    let lines = reader
        .lines()
        .map(|line| line.unwrap().trim().to_string())
        .collect::<Vec<_>>();
    
    // Get player decks
    let mut player1 = lines
        .clone()
        .into_iter()
        // Player 1:
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let mut player2 = lines
        .into_iter()
        // Player 1:, empty line, Player 2:
        .skip(player1.len() + 3)
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();

    loop {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();
        if card1 > card2 { 
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
        if player1.len() == 0 || player2.len() == 0 {
            break;
        }
    }

    if player1.len() == 0 {
        println!("{}", calculate_score(player2));
    } else {
        println!("{}", calculate_score(player1));
    }
}

fn calculate_score(deck: VecDeque<usize>) -> usize {
    deck
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, card)| {
            acc + (index + 1) * card
        })
}