use itertools::Itertools;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::io::prelude::*;
use std::io::BufReader;
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
};

#[derive(PartialEq, Eq)]
enum Player {
    Player1,
    Player2,
}

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

    // play_combat(player1.clone(), player2.clone());
    let winner = recursive_combat(
        &mut player1,
        &mut player2,
        1
    );

    if winner == Player::Player1 {
        log::debug!("{:?}", player1);
        println!("{}", calculate_score(player1.clone()));
    } else {
        log::debug!("{:?}", player2);
        println!("{}", calculate_score(player2.clone()));
    }
}

fn calculate_score(deck: VecDeque<usize>) -> usize {
    deck.into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, card)| acc + (index + 1) * card)
}

fn get_hash(deck: &VecDeque<usize>) -> String {
    deck.into_iter().map(|x| x.to_string()).join(",")
}

fn recursive_combat(
    player1: &mut VecDeque<usize>,
    player2: &mut VecDeque<usize>,
    game_count: u64,
) -> Player {
    // println!("Game {}, player 1 has {}, player 2 has {}", game_count, player1.len(), player2.len());
    let mut player1_hashes = HashSet::new();
    let mut player2_hashes = HashSet::new();
    let mut round_number = 1;
    loop {
        log::debug!("Player 1's deck: {:?}", player1);
        log::debug!("Player 2's deck: {:?}", player2);
        let player1_hash = get_hash(&player1);
        let player2_hash = get_hash(&player2);
        // If the deck state is duplicate
        if player1_hashes.contains(&player1_hash) && player2_hashes.contains(&player2_hash) {
            if player1_hashes.len() < 5 {
                println!("Player1 hash {}, Player2 hash {}, 1 hashes {:?}, 2 hashes {:?}", 
                player1_hash, player2_hash, player1_hashes, player2_hashes);
            }
            log::debug!("The winner of game {} is player 1!", game_count);
            return Player::Player1;
        }
        player1_hashes.insert(player1_hash);
        player2_hashes.insert(player2_hash);
        // println!("{:?}, {:?}", player1, player2);
        let player1_card = player1.pop_front().unwrap();
        let player2_card = player2.pop_front().unwrap();
        // If we should recurse
        if player1.len() >= player1_card && player2.len() >= player2_card {
            let winner = recursive_combat(
                &mut player1.clone().into_iter().take(player1_card).collect(),
                &mut player2.clone().into_iter().take(player2_card).collect(),
                game_count + 1
            );
            if winner == Player::Player1 {
                log::debug!("Player 1 wins round {} of game {}!", round_number, game_count);
                player1.push_back(player1_card);
                player1.push_back(player2_card);
            } else {
                log::debug!("Player 2 wins round {} of game {}!", round_number, game_count);
                player2.push_back(player2_card);
                player2.push_back(player1_card);
            }
        } else {
            log::debug!("Player 1 plays {}", player1_card);
            log::debug!("Player 2 plays {}", player2_card);
            // If we should play a normal round
            if player1_card > player2_card {
                log::debug!("Player 1 wins round {} of game {}!", round_number, game_count);
                player1.push_back(player1_card);
                player1.push_back(player2_card);
            } else {
                log::debug!("Player 2 wins round {} of game {}!", round_number, game_count);
                player2.push_back(player2_card);
                player2.push_back(player1_card);
            }
        }

        // If a deck is empty
        if player1.len() == 0 {
            log::debug!("The winner of game {} is player 2!", game_count);
            return Player::Player2;
        } else if player2.len() == 0 {
            log::debug!("The winner of game {} is player 1!", game_count);
            return Player::Player1;
        }
        round_number += 1;
    }
}

#[allow(dead_code)]
fn play_combat(mut player1: VecDeque<usize>, mut player2: VecDeque<usize>) {
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
            return;
        }
    }
}
