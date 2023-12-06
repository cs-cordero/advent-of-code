use advent_of_code::*;
use std::cmp::min;

fn main() {
    let data = read_input_as_lines("2023/day01/src/input.txt");

    let part1 = get_calibration_value(&data);
    let part2 = get_calibration_value(
        &data
            .iter()
            .map(|line| {
                let mut new_line = Vec::<char>::new();
                let chars = line.chars().collect::<Vec<char>>();
                for (i, c) in line.chars().enumerate() {
                    if c.is_ascii_digit() {
                        new_line.push(c)
                    } else {
                        let l = chars.len();
                        let three_char = &chars[i..min(l, i + 3)].iter().collect::<String>();
                        let four_char = &chars[i..min(l, i + 4)].iter().collect::<String>();
                        let five_char = &chars[i..min(l, i + 5)].iter().collect::<String>();
                        if three_char == "one" {
                            new_line.push('1');
                        } else if three_char == "two" {
                            new_line.push('2');
                        } else if five_char == "three" {
                            new_line.push('3');
                        } else if four_char == "four" {
                            new_line.push('4');
                        } else if four_char == "five" {
                            new_line.push('5');
                        } else if three_char == "six" {
                            new_line.push('6');
                        } else if five_char == "seven" {
                            new_line.push('7');
                        } else if five_char == "eight" {
                            new_line.push('8');
                        } else if four_char == "nine" {
                            new_line.push('9');
                        }
                    }
                }
                new_line.iter().collect()
            })
            .collect::<Vec<_>>(),
    );

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn get_calibration_value(values: &[String]) -> u32 {
    values
        .iter()
        .map(|value| {
            let mut left = Option::<u32>::None;
            let mut right = Option::<u32>::None;
            for char in value.chars() {
                if char.is_ascii_digit() {
                    if left.is_none() {
                        left = char.to_digit(10);
                    }
                    right = char.to_digit(10);
                }
            }
            left.unwrap() * 10 + right.unwrap()
        })
        .sum::<u32>()
}
