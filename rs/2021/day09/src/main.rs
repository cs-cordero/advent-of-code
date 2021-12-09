use advent_of_code::*;
use std::collections::{HashSet, VecDeque};

fn main() {
    let data = read_input_as_lines("2021/day09/src/input.txt")
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|number| number.to_string().parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let low_points = {
        let mut result = Vec::new();
        for (row_index, row) in data.iter().enumerate() {
            for (col_index, value) in row.iter().enumerate() {
                if adjacent_values(&data, (row_index, col_index))
                    .iter()
                    .all(|adjacent_num| *adjacent_num > *value)
                {
                    result.push((row_index, col_index));
                }
            }
        }
        result
    };

    let answer1 = low_points
        .iter()
        .map(|(row, col)| *data.get(*row).unwrap().get(*col).unwrap() as i32 + 1)
        .sum::<i32>();

    let answer2 = {
        let mut basin_sizes = low_points
            .iter()
            .map(|point| get_size(&data, *point))
            .collect::<Vec<_>>();
        basin_sizes.sort_unstable();
        basin_sizes.reverse();
        basin_sizes.iter().take(3).fold(1, |acc, num| acc * *num)
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

#[inline]
fn adjacent_points(data: &[Vec<u8>], location: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);
    let max_row_size = data.len();
    let max_col_size = data.iter().next().unwrap().len();
    let (row, col) = location;

    if let Some((prev_row, col)) = row.checked_sub(1).map(|prev_row| (prev_row, col)) {
        result.push((prev_row, col));
    }
    if let Some((row, prev_col)) = col.checked_sub(1).map(|prev_col| (row, prev_col)) {
        result.push((row, prev_col));
    }
    if row + 1 < max_row_size {
        result.push((row + 1, col));
    }
    if col + 1 < max_col_size {
        result.push((row, col + 1));
    }

    result
}

#[inline]
fn adjacent_values(data: &[Vec<u8>], location: (usize, usize)) -> Vec<u8> {
    adjacent_points(data, location)
        .into_iter()
        .map(|(row, col)| *data.get(row).and_then(|row| row.get(col)).unwrap())
        .collect()
}

fn get_size(data: &[Vec<u8>], location: (usize, usize)) -> i32 {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut size = 0;
    queue.push_back(location);

    while !queue.is_empty() {
        let (row, col) = queue.pop_front().unwrap();
        if seen.contains(&(row, col)) {
            continue;
        } else {
            seen.insert((row, col));
        }

        let value = *data.get(row).and_then(|row| row.get(col)).unwrap();
        if value == 9 {
            continue;
        }

        size += 1;
        adjacent_points(data, (row, col))
            .into_iter()
            .for_each(|point| queue.push_back(point));
    }

    size
}
