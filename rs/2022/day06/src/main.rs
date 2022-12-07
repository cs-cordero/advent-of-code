use std::collections::HashMap;
use advent_of_code::*;

fn main() {
    let input = read_input_as_string("2022/day06/src/input.txt").chars().collect::<Vec<_>>();

    let solution1 = solve(&input, 4).unwrap();
    let solution2 = solve(&input, 14).unwrap();

    println!("Part 1: {}", solution1);
    println!("Part 2: {}", solution2);
}

fn solve(input: &[char], required_length: usize) -> Option<usize> {
    let mut counts = HashMap::new();
    let mut slow: usize = 0;
    let mut fast: usize = 0;

    while fast < required_length {
        let next_char = input.get(fast).unwrap();
        counts.entry(next_char).and_modify(|entry| *entry += 1).or_insert(1);
        fast += 1;
    }

    while fast < input.len() {
        if counts.keys().len() == required_length {
            return Some(fast);
        }

        let prev_char = input.get(slow).unwrap();
        let next_char = input.get(fast).unwrap();

        counts.entry(next_char).and_modify(|entry| *entry += 1).or_insert(1);
        counts.entry(prev_char).and_modify(|entry| *entry -= 1);

        if *counts.get(&prev_char).unwrap() == 0 {
            counts.remove(&prev_char);
        }

        fast += 1;
        slow += 1;
    }

    None
}
