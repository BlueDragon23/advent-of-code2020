use log::LevelFilter;
use simple_logger::SimpleLogger;
use std::io::prelude::*;
use std::io::BufReader;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

#[derive(Clone, Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Error)
        .init()
        .unwrap();
    let f = File::open("input/input21_1.txt").unwrap();
    let reader = BufReader::new(f);
    let foods = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();
    let ingredients = get_ingredients(&foods);
    // For each ingredient, which allergens are possible
    // An allergen is possible if it appears in a food with that ingredient
    // An allergen becomes impossible if it appears in a food without that ingredient
    // If an ingredient has more than one allergen, check other ingredients

    // Plan: for each food, for each ingredient, add all allergens to impossibilities. O(nm)
    let (possibilities, _): (
        HashMap<String, HashSet<String>>,
        HashMap<String, HashSet<String>>,
    ) = foods.clone().into_iter().fold(
        (HashMap::new(), HashMap::new()),
        |(possibilities, impossibilities), food| {
            update_possibilities(&food, &ingredients, possibilities, impossibilities)
        },
    );
    println!("{:?}", count_appearances(&possibilities, &foods.clone()));
    // Evaluate manually, there's 8 choices
    // mfp,mgvfmvp,nhdjth,hcdchl,dvkbjh,dcvrf,bcjz,mhnrqpg is the solution
    println!(
        "{:?}",
        possibilities
            .into_iter()
            .filter(|(_, values)| values.len() > 0)
            .collect::<HashMap<String, HashSet<String>>>()
    );
}

fn count_appearances(possibilities: &HashMap<String, HashSet<String>>, foods: &Vec<Food>) -> u32 {
    possibilities
        .into_iter()
        .filter(|(_, values)| values.len() == 0)
        .fold(0, |acc, (ingredient, _)| {
            acc + foods.into_iter().fold(0, |i_count, food| {
                i_count
                    + if food.ingredients.contains(ingredient) {
                        1
                    } else {
                        0
                    }
            })
        })
}

fn update_possibilities(
    food: &Food,
    ingredients: &HashSet<String>,
    mut possibilities: HashMap<String, HashSet<String>>,
    mut impossibilities: HashMap<String, HashSet<String>>,
) -> (
    HashMap<String, HashSet<String>>,
    HashMap<String, HashSet<String>>,
) {
    ingredients.into_iter().for_each(|ingredient| {
        if food.ingredients.contains(ingredient) {
            // Add to possibilities if not in impossibilities
            possibilities
                .entry(ingredient.clone().to_string())
                .or_insert(HashSet::new())
                .extend(food.allergens.clone().into_iter().filter(|allergen| {
                    !impossibilities
                        .get(&ingredient.clone())
                        .unwrap_or(&HashSet::new())
                        .contains(allergen)
                }));
        } else {
            // Add to impossibilities
            impossibilities
                .entry(ingredient.clone().to_string())
                .or_insert(HashSet::new())
                .extend(food.allergens.clone());
            // And remove from possibilities
            possibilities
                .entry(ingredient.clone().to_string())
                .and_modify(|set| {
                    for a in food.allergens.clone() {
                        set.remove(&a);
                    }
                });
        }
    });
    (possibilities, impossibilities)
}

fn get_ingredients(foods: &Vec<Food>) -> HashSet<String> {
    foods
        .into_iter()
        .fold(HashSet::new(), |mut ingredients, food| {
            ingredients.extend(food.ingredients.clone());
            ingredients
        })
}

// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
fn parse_line(line: String) -> Food {
    line.split_once("(contains")
        .map(|(ingredients, allergens)| {
            // Ingredients
            let ingredient_set = ingredients
                .split_ascii_whitespace()
                .map(|s| s.trim().to_string())
                .collect::<_>();
            // Allergens
            let allergens_set = allergens
                .strip_suffix(")")
                .map(|s| s.split(", ").map(|s| s.trim().to_string()).collect())
                .unwrap_or(HashSet::new());
            Food {
                ingredients: ingredient_set,
                allergens: allergens_set,
            }
        })
        .unwrap()
}
