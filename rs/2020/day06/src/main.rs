use advent_of_code::*;
use std::collections::HashSet;

fn main() {
    let groups = read_input_as_string("2020/day06/src/input.txt")
        .split("\n\n")
        .map(|group| group.to_owned())
        .collect::<Vec<_>>();

    let answer1: u32 = groups
        .iter()
        .map(|group| group
            .split('\n')
            .flat_map(|answers| answers.chars())
            .collect::<HashSet<_>>()
        )
        .map(|unique_answers_for_group| unique_answers_for_group.len() as u32)
        .sum();

    let answer2: u32 = groups
        .iter()
        .map(|group| group
            .split('\n')
            .map(|answers_for_single_person| answers_for_single_person.chars().collect::<HashSet<_>>())
        )
        .map(|mut group| {
            let first_person = group.next().unwrap();
            group.fold(first_person, |a, b| a.intersection(&b).cloned().collect())
        })
        .map(|common_answers_per_group| common_answers_per_group.len() as u32)
        .sum();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
