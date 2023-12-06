use std::cmp::min;
use std::collections::HashSet;

use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2023/day04/src/input.txt")
        .into_iter()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (winning, play) = line.split_once(" | ").unwrap();
            let winning = winning
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            let play = play
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<HashSet<_>>();

            (winning, play)
        })
        .collect::<Vec<_>>();

    let part1 = data
        .iter()
        .map(|(winning, played)| {
            let match_count = winning.intersection(played).count();
            if match_count == 0 {
                0
            } else {
                2u32.pow(match_count as u32 - 1)
            }
        })
        .sum::<u32>();

    let part2 = {
        let mut card_counts = Vec::new();
        card_counts.resize(data.len(), 1);

        for (i, (winning, played)) in data.iter().enumerate() {
            let match_count = winning.intersection(played).count();
            for j in i + 1..min(data.len(), i + 1 + match_count) {
                card_counts[j] += card_counts[i];
            }
        }

        card_counts.iter().sum::<u32>()
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
