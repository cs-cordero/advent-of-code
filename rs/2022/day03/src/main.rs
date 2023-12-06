use advent_of_code::*;
use std::collections::HashSet;

fn main() {
    let input = read_input_as_lines("2022/day03/src/input.txt");

    let part1: i32 = input
        .iter()
        .map(|rucksack| {
            let length = rucksack.len();

            let (left, right) = rucksack.split_at(length / 2);
            let left = left.chars().collect::<HashSet<_>>();
            let right = right.chars().collect::<HashSet<_>>();

            find_priority(left.intersection(&right).collect())
        })
        .sum();

    let part2: i32 = input
        .chunks_exact(3)
        .map(|rucksack| {
            let elf1 = rucksack[0].chars().collect::<HashSet<_>>();
            let elf2 = rucksack[1].chars().collect::<HashSet<_>>();
            let elf3 = rucksack[2].chars().collect::<HashSet<_>>();
            find_priority(
                elf1.intersection(&elf2.intersection(&elf3).copied().collect())
                    .collect(),
            )
        })
        .sum();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn find_priority(items: HashSet<&char>) -> i32 {
    items.into_iter().copied().fold(0, |acc, item| {
        if item.is_lowercase() {
            acc + item as i32 - 96
        } else if item.is_uppercase() {
            acc + item as i32 - 64 + 26
        } else {
            panic!("Failed to find priority of item");
        }
    })
}
