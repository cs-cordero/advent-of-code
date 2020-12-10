use advent_of_code::*;
use std::cmp::min;

fn main() {
    let input = {
        let mut v = read_input_as_lines("2020/day10/src/input.txt")
            .into_iter()
            .map(|line| line.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        v.push(0);
        v.push(v.iter().max().unwrap() + 3);
        v.sort_unstable();
        v
    };

    let answer1 = {
        let len = input.len();
        let mut diff_of_one_count = 0;
        let mut diff_of_three_count = 0;
        input[..len - 1]
            .iter()
            .zip(input[1..len].iter())
            .for_each(|(prev, curr)| match curr - prev {
                3 => diff_of_three_count += 1,
                1 => diff_of_one_count += 1,
                _ => (),
            });
        diff_of_three_count * diff_of_one_count
    };

    let answer2 = {
        let size = input.len();
        let mut dp = {
            let mut v = vec![0_i64; size];
            *v.last_mut().unwrap() = 1;
            v
        };

        for (index, value) in input.iter().enumerate().rev().skip(1) {
            let bound = min(index + 3, size - 1);

            dp[index] = ((index + 1)..=bound)
                .filter_map(|other_index| {
                    input
                        .get(other_index)
                        .zip(dp.get(other_index))
                        .filter(|(other_value, _)| *other_value - value <= 3)
                        .map(|(_, dp_value)| dp_value)
                })
                .sum();
        }
        *dp.first().unwrap()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
