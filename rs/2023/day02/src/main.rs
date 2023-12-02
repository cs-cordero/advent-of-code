use std::cmp::{max, min};
use advent_of_code::*;

struct BallPull {
    red: u32,
    green: u32,
    blue: u32
}

fn main() {
    let data = read_input_as_lines("2023/day02/src/input.txt")
        .into_iter()
        .map(|line| {
            let (_, balls) = line.split_once(": ").unwrap();
            balls.split("; ")
                .map(|ball_pull| {
                    let ball_pulls = ball_pull.split(", ").map(|s| s.split_once(" ").unwrap());
                    let mut result = BallPull {
                        red: 0,
                        blue: 0,
                        green: 0
                    };
                    for (count, color) in ball_pulls.into_iter() {
                        let count = count.parse::<u32>().unwrap();
                        match color {
                            "red" => result.red = count,
                            "blue" => result.blue = count,
                            "green" => result.green = count,
                            _ => panic!("oops")
                        }
                    }
                    result
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let part1 = {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        data.iter().enumerate()
            .filter(|(_, pulls)| {
                pulls.iter().all(|colors| colors.red <= max_red && colors.green <= max_green && colors.blue <= max_blue)
            })
            .map(|(game, _)| (game + 1) as u32)
            .sum::<u32>()
    };

    let part2 = {
        data.iter().map(|game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            game.iter().for_each(|colors| {
                min_red = max(min_red, colors.red);
                min_green = max(min_green, colors.green);
                min_blue = max(min_blue, colors.blue);
            });

            min_red * min_green * min_blue
        })
        .sum::<u32>()
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn get_calibration_value(values: &[String]) -> u32 {
    values.iter()
        .map(|value| {
            let mut left = Option::<u32>::None;
            let mut right = Option::<u32>::None;
            for char in value.chars() {
                if char.is_digit(10) {
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
