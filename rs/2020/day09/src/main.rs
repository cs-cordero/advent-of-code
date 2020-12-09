use advent_of_code::*;
use std::cmp::Ordering;
use std::collections::{BTreeMap, VecDeque};

fn main() {
    let input = read_input_as_lines("2020/day09/src/input.txt")
        .into_iter()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let answer1 = part_1(&input);
    let answer2 = part_2(&input, answer1);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn part_1(values: &[u64]) -> u64 {
    let mut queue = values.iter().copied().take(25).collect::<VecDeque<_>>();
    let mut window = {
        let mut map = BTreeMap::<u64, u8>::new();
        queue.iter().for_each(|value| {
            *map.entry(*value).or_insert(0) += 1;
        });
        map
    };

    for value in values.iter().copied().skip(25) {
        let window_as_slice = window
            .iter()
            .flat_map(|(key, count)| (0..*count).into_iter().map(move |_| *key))
            .collect::<Vec<_>>();

        if !is_valid(&window_as_slice, value) {
            return value;
        }

        let value_to_remove = queue.pop_front().unwrap();
        queue.push_back(value);

        if let Some(count) = window.get_mut(&value_to_remove) {
            *count -= 1;
            if *count == 0 {
                window.remove(&value_to_remove).unwrap_or_else(|| {
                    panic!(
                        "Failed to remove value [{}] from BTreeMap.",
                        value_to_remove
                    )
                });
            }
        }
        *window.entry(value).or_insert(0) += 1;
    }

    panic!("Unable to solve part 1!");
}

fn part_2(values: &[u64], target: u64) -> u64 {
    let prefix_sums = {
        let mut v = Vec::with_capacity(values.len() + 1);
        v.push(0);
        for value in values {
            v.push(v.last().unwrap() + value);
        }
        v
    };

    let mut slow: usize = 0;
    let mut fast: usize = 0;
    while fast < values.len() {
        let current_value = prefix_sums[fast] - prefix_sums[slow];
        match current_value.cmp(&target) {
            Ordering::Equal => {
                return values[slow..fast].iter().min().unwrap()
                    + values[slow..fast].iter().max().unwrap();
            }
            Ordering::Greater => slow += 1,
            Ordering::Less => fast += 1,
        }
    }
    0
}

fn is_valid(sorted_nums: &[u64], target: u64) -> bool {
    let mut left = 0;
    let mut right = sorted_nums.len() - 1;
    while left < right {
        let sum = sorted_nums[left] + sorted_nums[right];
        match sum.cmp(&target) {
            Ordering::Greater => right -= 1,
            Ordering::Less => left += 1,
            Ordering::Equal => return true,
        }
    }
    false
}
