use advent_of_code::*;
use std::collections::{HashSet, VecDeque};

fn main() {
    let mut data = read_input_as_lines("2021/day11/src/input.txt")
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let row_count = data.len();
    let col_count = data.get(0).unwrap().len();

    let mut answer1 = 0;

    let mut step_count = 0;
    let answer2 = loop {
        let flash_count = step(&mut data);
        step_count += 1;

        if flash_count == row_count * col_count {
            break step_count;
        }

        if step_count <= 100 {
            answer1 += flash_count;
        }
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn step(octopuses: &mut [Vec<u8>]) -> usize {
    let mut queue = VecDeque::new();
    let mut flashed = HashSet::new();

    // increase energy
    for (r, row) in octopuses.iter_mut().enumerate() {
        for (c, octopus) in row.iter_mut().enumerate() {
            *octopus += 1; // could overflow
            if *octopus > 9 {
                // octopus flashes
                queue.push_back((r, c));
                flashed.insert((r, c));
            }
        }
    }

    // flash the octopuses
    let row_limit = octopuses.len();
    let col_limit = octopuses.iter().next().unwrap().len();
    while !queue.is_empty() {
        let flash_point = queue.pop_front().unwrap();
        for (row, col) in get_adjacent_points(flash_point, (row_limit, col_limit)) {
            let point = (row, col);
            if flashed.contains(&point) {
                continue;
            }

            let adjacent_octopus = octopuses.get_mut(row).unwrap().get_mut(col).unwrap();
            *adjacent_octopus += 1;
            if *adjacent_octopus > 9 {
                queue.push_back(point);
                flashed.insert(point);
            }
        }
    }

    // reset the flashed octopuses.
    for row in octopuses {
        for octopus in row {
            if *octopus > 9 {
                *octopus = 0;
            }
        }
    }

    flashed.len()
}

fn get_adjacent_points(
    point: (usize, usize),
    limits_non_inclusive: (usize, usize),
) -> Vec<(usize, usize)> {
    let (row, col) = point;
    let (row_limit, col_limit) = limits_non_inclusive;
    let prev_row = row.checked_sub(1);
    let next_row = row.checked_add(1).filter(|value| *value < row_limit);
    let prev_col = col.checked_sub(1);
    let next_col = col.checked_add(1).filter(|value| *value < col_limit);
    let row = Some(row);
    let col = Some(col);

    let mut result = Vec::new();
    for r in [prev_row, row, next_row] {
        if r.is_none() {
            continue;
        }
        for c in [prev_col, col, next_col] {
            if r == row && c == col {
                continue;
            }
            if c.is_none() {
                continue;
            }

            result.push((r.unwrap(), c.unwrap()));
        }
    }
    result
}
