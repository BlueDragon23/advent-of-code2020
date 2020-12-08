use petgraph::Graph;
use std::collections::HashSet;
use petgraph::prelude::Bfs;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use petgraph::{graph::{DiGraph, NodeIndex}};
use reformation::Reformation;
use regex::{Regex};
use petgraph::dot::{Dot, Config};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Reformation)]
#[reformation(r"{colour} bags?\.?")]
struct Node {
    colour: String
}

fn main() {
    let f = File::open("input/input7_1.txt").unwrap();
    let reader = BufReader::new(f);
    let edge_regex = Regex::new(r"(\d) (.+) bags?\.?").unwrap();
    let (graph, _) = reader.lines().map(|line| line.unwrap())
        .fold((DiGraph::new(), HashMap::new()), |(mut graph, mut map): (_, HashMap<Node, NodeIndex>), line| {
            let mut parts = line.split(" contain ");
            let from = Node::parse(parts.next().unwrap()).unwrap();
            let from_index = *map.entry(from.clone()).or_insert(graph.add_node(from.clone()));
            let optional_to = parts.next().unwrap();
            if optional_to == "no other bags." {
                return (graph, map)
            }
            optional_to.split(", ").for_each(|bag_edge| {
                let captures = edge_regex.captures(bag_edge).unwrap();
                let weight = captures[1].parse::<usize>().unwrap();
                let to = Node { colour: captures[2].to_string() };
                let to_index = *map.entry(to.clone()).or_insert(graph.add_node(to.clone()));
                graph.add_edge(from_index, to_index, weight);
            });
            (graph, map)
        });
    
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let index = graph.node_indices().find(|i| graph[*i].colour == "shiny gold").unwrap();
    let sum = part2(graph, index);
    println!("{}", sum);
}

fn part1(graph: Graph<Node, usize>) -> i32 {
    // This is not my original part 1, the graph edges need to be reversed. CBF right now
    let index = graph.node_indices().find(|i| graph[*i].colour == "shiny gold").unwrap();
    let mut bfs = Bfs::new(&graph, index);
    let mut visited = HashSet::new();
    visited.insert(index);
    let mut count = 0;
    while let Some(node) = bfs.next(&graph) {
        if !visited.contains(&node) {
            count += 1;
            visited.insert(node);
        }
    }
    count
}

fn part2(graph: Graph<Node, usize>, node: NodeIndex) -> i32 {
    // Multiply edge weight by number of bags inside it
    let mut walker = graph.neighbors(node).detach();
    let mut sum = 0;
    while let Some((edge_index, node_index)) = walker.next(&graph) {
        let weight = (*graph.edge_weight(edge_index).unwrap()) as i32;
        // For each node, bags contained = n bags * m bags inside them
        let children = part2(graph.clone(), node_index);
        if children == 0 {
            // Leaf node
            println!("{:?} contains {} {:?} bags", graph.node_weight(node), weight, graph.node_weight(node_index));
            sum += weight
        } else {
            println!("{:?} is parent of {} {:?} bags with {} children", graph.node_weight(node), weight, graph.node_weight(node_index), children);
            sum += weight + weight * children
        }
    }
    sum
}