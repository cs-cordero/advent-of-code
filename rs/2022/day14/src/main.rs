extern crate core;

use advent_of_code::*;
use std::cmp::{max, min};
use std::collections::HashSet;

fn main() {
    let input = {
        let mut rocks = HashSet::new();
        for line in read_input_as_lines("2022/day14/src/input.txt") {
            let points: Vec<(usize, usize)> = line
                .split(" -> ")
                .map(|chunk| {
                    let (x, y) = chunk.split_once(',').unwrap();
                    let x: usize = x.parse().unwrap();
                    let y: usize = y.parse().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>();

            for window in points.windows(2) {
                let (ax, ay) = window[0];
                let (bx, by) = window[1];
                assert!(ax == bx || ay == by);

                for x in min(ax, bx)..=max(ax, bx) {
                    rocks.insert((x, ay));
                    rocks.insert((x, by));
                }

                for y in min(ay, by)..=max(ay, by) {
                    rocks.insert((ax, y));
                    rocks.insert((bx, y));
                }
            }
        }
        rocks
    };

    let deepest_y = input.iter().map(|(_, y)| *y).max().unwrap();

    let solution1 = {
        let mut rocks = input.iter().copied().collect::<HashSet<_>>();
        let rocks_count = input.len();
        while let Some((new_sand_x, new_sand_y)) = simulate_sand(&rocks, deepest_y) {
            rocks.insert((new_sand_x, new_sand_y));
        }
        rocks.len() - rocks_count
    };

    let solution2 = {
        let mut rocks = input.iter().copied().collect::<HashSet<_>>();

        for x in 0..=100000 {
            rocks.insert((x, deepest_y + 2));
        }

        let rocks_count = rocks.len();

        while let Some((new_sand_x, new_sand_y)) = simulate_sand(&rocks, deepest_y + 5) {
            rocks.insert((new_sand_x, new_sand_y));
        }
        rocks.len() - rocks_count
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn simulate_sand(rocks: &HashSet<(usize, usize)>, deepest_y: usize) -> Option<(usize, usize)> {
    let mut sand_x: usize = 500;
    let mut sand_y: usize = 0;

    loop {
        if sand_y >= deepest_y {
            return None;
        }

        if !rocks.contains(&(sand_x, sand_y + 1)) {
            // immediately below is clear
            sand_y += 1;
            continue;
        } else if !rocks.contains(&(sand_x - 1, sand_y + 1)) {
            // down-left is clear
            sand_x -= 1;
            sand_y += 1;
            continue;
        } else if !rocks.contains(&(sand_x + 1, sand_y + 1)) {
            // down-right is clear
            sand_x += 1;
            sand_y += 1;
            continue;
        } else {
            // totally blocked, sand must now rest
            break;
        }
    }

    let rest_point = (sand_x, sand_y);
    if rocks.contains(&rest_point) {
        None
    } else {
        Some(rest_point)
    }
}
