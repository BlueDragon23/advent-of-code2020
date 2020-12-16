use std::{collections::HashSet, fs::File};
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;
use std::collections::HashMap;
use std::iter::FromIterator;

struct State {
    parse_state: ParseState,
    fields: Vec<Field>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>
}

#[derive(Clone, Debug)]
enum ParseState {
    Rules,
    MyTicket,
    Nearby
}

#[derive(Clone, Debug)]
struct Field {
    name: String,
    lower_range: (usize, usize),
    upper_range: (usize, usize),
}

#[derive(Clone, Debug)]
struct Ticket {
    values: Vec<usize>
}

fn main() {
    let f = File::open("input/input16_1.txt").unwrap();
    let reader = BufReader::new(f);
    let parsed = reader
        .lines()
        .map(|line| line.unwrap())
        .fold(State {
            parse_state: ParseState::Rules,
            fields: vec![],
            my_ticket: Ticket { values: vec![] },
            tickets: vec![]
        }, |mut state, line| {
            if line.is_empty() {
                let next_state = match state.parse_state {
                    ParseState::Rules => ParseState::MyTicket,
                    ParseState::MyTicket => ParseState::Nearby,
                    _ => panic!("no")
                };
                return State { parse_state: next_state, ..state };
            }
            match state.parse_state {
                ParseState::Rules => {
                    let new_field = parse_field(line.as_str());
                    state.fields.push(new_field);
                    state
                },
                ParseState::MyTicket => {
                    state.my_ticket = parse_ticket(line.as_str());
                    state
                },
                ParseState::Nearby => {
                    let new_ticket = parse_ticket(line.as_str());
                    state.tickets.push(new_ticket);
                    state
                }
            }
        });
    let valid_range: (usize, usize) = (28, 971);
    let invalid_total = calculate_invalid_total(parsed.tickets.clone(), valid_range);
    println!("part 1: {}", invalid_total);
    let valid = parsed.tickets
        .into_iter()
        .filter(|ticket| ticket.values
            .iter()
            .all(|&x| x >= valid_range.0 && x <= valid_range.1)
        )
        .collect::<Vec<_>>();
    // Find which field maps to which column
    let field_columns = parsed.fields
        .into_iter()
        .fold(HashMap::new(), |mut mapping, field| {
            for column in 0..20 { // Number of fields
                if valid.iter().all(|ticket| {
                    let x = ticket.values[column];
                    x >= field.lower_range.0 && x <= field.lower_range.1 || 
                    x >= field.upper_range.0 && x <= field.upper_range.1
                }) {
                    mapping.entry(field.name.clone()).or_insert(vec![]).push(column);
                }
            }
            mapping
        });
    let mut names = field_columns.keys().collect::<Vec<_>>();
    names.sort_by(|&a, &b| field_columns[a].len().partial_cmp(&field_columns[b].len()).unwrap());
    let final_mapping = names.iter().fold(HashMap::new(), |mut m, &name| {
        let options = field_columns[name]
            .iter()
            .filter(|&a| !m.values().any(|&x| x == a))
            .collect::<Vec<_>>();
        if options.len() == 1 {
            m.insert(name, options[0]);
        } else {
            panic!("Found {:?} options for {}", options, name);
        }
        m
    });
    let my_ticket = parsed.my_ticket;
    let product = final_mapping
        .into_iter()
        .filter(|entry| entry.0.starts_with("departure"))
        .map(|entry| my_ticket.values[*entry.1])
        .product::<usize>();
    println!("part 2: {}", product);
}

fn parse_field(line: &str) -> Field {
    let ranges_regex = Regex::new(r"(\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let mut parts = line.split(": ").into_iter();
    let field_name = parts.next().unwrap();
    let ranges = parts.next().unwrap_or_else(|| panic!("Failed on line {}", line));
    let range_captures = ranges_regex.captures(ranges).unwrap();
    Field { 
        name: field_name.to_string(), 
        lower_range: (range_captures[1].parse().unwrap(), range_captures[2].parse().unwrap()), 
        upper_range: (range_captures[3].parse().unwrap(), range_captures[4].parse().unwrap()),
    }
}

fn parse_ticket(line: &str) -> Ticket {
    Ticket { values: line.split(",").map(|n| n.parse().unwrap()).collect() }
}

fn calculate_invalid_total(tickets: Vec<Ticket>, valid_range: (usize, usize)) -> usize {
    tickets
        .into_iter()
        .map(|ticket| ticket.values
            .into_iter()
            .filter(|&x| x < valid_range.0 || x > valid_range.1)
            .sum::<usize>()
        )
        .sum::<usize>()
}