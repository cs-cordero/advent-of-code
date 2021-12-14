use advent_of_code::*;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let (mut pair_counts, rules) = {
        let raw = read_input_as_string("2021/day14/src/input.txt");
        let (raw_template, raw_rules) = raw.split_once("\n\n").unwrap();

        let pair_counts = raw_template
            .trim()
            .chars()
            .collect::<Vec<_>>()
            .windows(2)
            .map(|window| (window.iter().collect::<String>(), 1usize))
            .fold(HashMap::new(), |mut acc, (pair, freq)| {
                *acc.entry(pair).or_insert(0) += freq;
                acc
            });

        let rules = raw_rules
            .lines()
            .map(|line| {
                let (source, target) = line.split_once(" -> ").unwrap();
                (source.to_string(), target.chars().next().unwrap())
            })
            .collect::<HashMap<String, char>>();

        (pair_counts, rules)
    };

    let answer1 = {
        for _ in 0..10 {
            pair_counts = perform_step(&pair_counts, &rules);
        }
        get_answer(&pair_counts)
    };

    let answer2 = {
        for _ in 10..40 {
            pair_counts = perform_step(&pair_counts, &rules);
        }
        get_answer(&pair_counts)
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

#[derive(Debug)]
struct SmartCounter {
    left: usize,
    right: usize,
}

impl SmartCounter {
    fn new() -> Self {
        Self { left: 0, right: 0 }
    }

    fn increment_left(&mut self, amount: usize) {
        self.left += amount;
    }

    fn increment_right(&mut self, amount: usize) {
        self.right += amount;
    }

    fn get_count(&self) -> usize {
        max(self.left, self.right)
    }
}

fn perform_step(
    pair_frequency: &HashMap<String, usize>,
    rules: &HashMap<String, char>,
) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (pair, frequency) in pair_frequency {
        let rule = rules.get(pair).unwrap();
        let left = format!("{}{}", &pair[0..1], rule);
        let right = format!("{}{}", rule, &pair[1..2]);
        *result.entry(left).or_insert(0) += frequency;
        *result.entry(right).or_insert(0) += frequency;
    }
    result
}

fn count_chars_from_pair_counts(pair_counts: &HashMap<String, usize>) -> HashMap<char, usize> {
    pair_counts
        .iter()
        .fold(HashMap::new(), |mut acc, (pair, frequency)| {
            let mut pair_chars = pair.chars();
            let left = pair_chars.next().unwrap();
            let right = pair_chars.next().unwrap();
            acc.entry(left)
                .or_insert_with(SmartCounter::new)
                .increment_left(*frequency);
            acc.entry(right)
                .or_insert_with(SmartCounter::new)
                .increment_right(*frequency);
            acc
        })
        .into_iter()
        .map(|(char, smart_counter)| (char, smart_counter.get_count()))
        .collect()
}

fn get_answer(pair_counts: &HashMap<String, usize>) -> usize {
    let freq = count_chars_from_pair_counts(pair_counts)
        .values()
        .copied()
        .collect::<Vec<_>>();

    let (min, max) = get_min_and_max(&freq);
    max - min
}
