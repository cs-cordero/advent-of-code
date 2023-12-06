use std::collections::HashSet;

use advent_of_code::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Number {
    value: u32,
    col_start: usize,
    col_end: usize,
    row: usize,
}

fn main() {
    let data = read_input_as_lines("2023/day03/src/input.txt");

    let mut numbers: HashSet<Number> = HashSet::new();
    let mut symbols: Vec<(usize, usize, char)> = Vec::new();

    for (row, line) in data.iter().enumerate() {
        let mut col_start: Option<usize> = None;
        let mut col_end: Option<usize> = None;
        let mut number: u32 = 0;

        for (col, value) in line.chars().enumerate() {
            if let Some(digit) = value.to_digit(10) {
                if col_start.is_none() {
                    col_start = Some(col);
                }
                col_end = Some(col);
                number *= 10;
                number += digit
            } else {
                if let (Some(col_start), Some(col_end)) = (col_start, col_end) {
                    let new_number = Number {
                        value: number,
                        col_start,
                        col_end,
                        row,
                    };
                    numbers.insert(new_number);
                }

                if value != '.' {
                    symbols.push((row, col, value));
                }

                col_start = None;
                col_end = None;
                number = 0;
            }
        }

        if let (Some(col_start), Some(col_end)) = (col_start, col_end) {
            let new_number = Number {
                value: number,
                col_start,
                col_end,
                row,
            };
            numbers.insert(new_number);
        }
    }

    let part1 = {
        let mut non_part_numbers = numbers.clone();

        for (row, col, _) in symbols.iter().copied() {
            let surrounding = [
                // previous row
                row.checked_sub(1).zip(col.checked_sub(1)),
                row.checked_sub(1).map(|value| (value, col)),
                row.checked_sub(1).map(|value| (value, col + 1)),
                // current row
                col.checked_sub(1).map(|value| (row, value)),
                Some((row, col + 1)),
                // next row
                col.checked_sub(1).map(|value| (row + 1, value)),
                Some((row + 1, col)),
                Some((row + 1, col + 1)),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

            for (other_row, other_col) in surrounding {
                let part_numbers = non_part_numbers
                    .iter()
                    .filter(|number| {
                        number.row == other_row
                            && number.col_start <= other_col
                            && other_col <= number.col_end
                    })
                    .copied()
                    .collect::<Vec<_>>();

                for to_remove in part_numbers.iter() {
                    non_part_numbers.remove(to_remove);
                }
            }
        }

        numbers
            .difference(&non_part_numbers)
            .map(|number| number.value)
            .sum::<u32>()
    };

    let part2 = {
        symbols
            .iter()
            .filter(|(_, _, char)| *char == '*')
            .filter_map(|(row, col, _)| {
                let surrounding = [
                    // previous row
                    row.checked_sub(1).zip(col.checked_sub(1)),
                    row.checked_sub(1).map(|value| (value, *col)),
                    row.checked_sub(1).map(|value| (value, col + 1)),
                    // current row
                    col.checked_sub(1).map(|value| (*row, value)),
                    Some((*row, col + 1)),
                    // next row
                    col.checked_sub(1).map(|value| (row + 1, value)),
                    Some((row + 1, *col)),
                    Some((row + 1, col + 1)),
                ]
                .into_iter()
                .filter_map(|v| {
                    v.and_then(|(row, col)| {
                        numbers.iter().find(|number| {
                            number.row == row && number.col_start <= col && col <= number.col_end
                        })
                    })
                })
                .copied()
                .collect::<HashSet<_>>();

                if surrounding.len() != 2 {
                    None
                } else {
                    Some(surrounding.iter().map(|num| num.value).product::<u32>())
                }
            })
            .sum::<u32>()
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
