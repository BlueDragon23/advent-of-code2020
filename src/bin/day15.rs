use std::collections::HashMap;

fn main() {
    let mut numbers = vec![7, 12, 1, 0, 16, 2];
    let stop_value = 30000000;
    let iterator = numbers.len()..stop_value;
    let mut indices = numbers
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, x)| (x, vec![idx]))
        .collect::<HashMap<i32, Vec<usize>>>();
    iterator.for_each(|index| {
        let prev = numbers[index - 1];
        if indices.get(&prev).map(|xs| xs.len() >= 2).unwrap_or(false) {
            // We have at least two indices, just use them
            let mut prev_indices = indices.get(&prev).unwrap().iter().rev();
            let next = (prev_indices.next().unwrap() - prev_indices.next().unwrap()) as i32;
            indices.entry(next).or_insert(vec![]).push(index);
            numbers.push(next);
        } else {
            // We don't have two instances of this
            numbers.push(0);
            indices.entry(0).or_insert(vec![]).push(index);
        }
    });
    println!("{}", numbers[2020 - 1]);
    println!("{}", numbers[30000000 - 1]);
}
