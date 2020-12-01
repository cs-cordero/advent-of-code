use advent_of_code::*;
use std::collections::HashSet;

fn main() {
    let values: Vec<i32> = read_input_as_lines("2020/day01/src/input.txt")
        .into_iter()
        .map(|val| val.parse::<i32>().unwrap())
        .collect();

    println!("Part 1: {}", two_sum(&values, 2020).unwrap());
    println!("Part 2: {}", three_sum(&values, 2020).unwrap());
}

fn two_sum(values: &[i32], target: i32) -> Option<i32> {
    let mut previous_values: HashSet<i32> = HashSet::new();
    for value in values {
        let diff = target - value;
        if previous_values.contains(&diff) {
            return Some(value * diff);
        }
        previous_values.insert(*value);
    }
    None
}

fn three_sum(values: &[i32], target: i32) -> Option<i32> {
    for i in 0..values.len()-2 {
        let num = values[i];
        if let Some(two_sum_result) = two_sum(&values[i+1..], target - num) {
            return Some(num * two_sum_result);
        }
    }
    None
}
