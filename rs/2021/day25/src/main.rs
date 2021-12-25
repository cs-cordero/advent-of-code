use advent_of_code::*;
use std::collections::HashSet;

type Point = (usize, usize);

fn main() {
    let cucumbers = read_input_as_lines("2021/day25/src/input.txt")
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let limits = get_limits(&cucumbers);
    let (mut down_cucumbers, mut right_cucumbers) = {
        let mut down = HashSet::new();
        let mut right = HashSet::new();

        for (row_index, row) in cucumbers.into_iter().enumerate() {
            for (col_index, cucumber) in row.into_iter().enumerate() {
                match cucumber {
                    'v' => {
                        down.insert((row_index, col_index));
                    }
                    '>' => {
                        right.insert((row_index, col_index));
                    }
                    '.' => {}
                    _ => panic!("Invalid cucumber representation"),
                }
            }
        }

        (down, right)
    };

    let answer1 = {
        let mut step_count = 0;
        while step(&mut down_cucumbers, &mut right_cucumbers, limits) {
            step_count += 1;
        }
        step_count + 1
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: Solved by having all other 49 stars.");
}

fn step(
    down_cucumbers: &mut HashSet<Point>,
    right_cucumbers: &mut HashSet<Point>,
    (row_limit, col_limit): Point,
) -> bool {
    let mut moved = false;

    *right_cucumbers = right_cucumbers
        .iter()
        .map(|(row, col)| {
            let destination = (*row, (col + 1) % col_limit);
            let destination_is_open =
                !down_cucumbers.contains(&destination) && !right_cucumbers.contains(&destination);
            if destination_is_open {
                moved = true;
                destination
            } else {
                (*row, *col)
            }
        })
        .collect();

    *down_cucumbers = down_cucumbers
        .iter()
        .map(|(row, col)| {
            let destination = ((row + 1) % row_limit, *col);
            let destination_is_open =
                !down_cucumbers.contains(&destination) && !right_cucumbers.contains(&destination);
            if destination_is_open {
                moved = true;
                destination
            } else {
                (*row, *col)
            }
        })
        .collect();

    moved
}
