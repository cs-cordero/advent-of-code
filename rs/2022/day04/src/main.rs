use std::ops::RangeInclusive;
use advent_of_code::*;

fn main() {
    let input = read_input_as_lines("2022/day04/src/input.txt")
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once(',').unwrap();
            (parse_into_range(left), parse_into_range(right))
        })
        .collect::<Vec<_>>();

    let part1: i32 = input.iter()
        .map(|(left, right)| {
            let left_contains_right = left.start() <= right.start() && left.end() >= right.end();
            let right_contains_left = right.start() <= left.start() && right.end() >= left.end();

            i32::from(left_contains_right || right_contains_left)
        })
        .sum();

    let part2: i32 = input.iter()
        .map(|(left, right)| {
            i32::from(left.start() <= right.end() && left.end() >= right.start())
        })
        .sum();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn parse_into_range(input: &str) -> RangeInclusive<i32> {
    input.split_once('-')
        .map(|(lower, upper)| lower.parse::<i32>().unwrap()..=upper.parse::<i32>().unwrap())
        .unwrap()
}
