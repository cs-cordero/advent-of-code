extern crate core;

use advent_of_code::*;
use std::collections::HashSet;

fn main() {
    let input = read_input_as_lines("2022/day08/src/input.txt")
        .iter()
        .map(|row| {
            row.chars()
                .map(|height| height.to_string().parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let solution1 = {
        let mut visible_trees = HashSet::new();

        for row_index in 0..input.len() {
            // from left
            let mut current_height: i32 = -1;
            for (col_index, tree) in input.get(row_index).unwrap().iter().enumerate() {
                if *tree > (current_height as isize) {
                    visible_trees.insert((row_index, col_index));
                    current_height = *tree as i32;
                }
            }

            // from right
            let mut current_height: i32 = -1;
            for (col_index, tree) in input.get(row_index).unwrap().iter().enumerate().rev() {
                if *tree > current_height as isize {
                    visible_trees.insert((row_index, col_index));
                    current_height = *tree as i32;
                }
            }
        }

        let col_len = input.first().unwrap().len();
        for col_index in 0..col_len {
            // from top
            let mut current_height: i32 = -1;
            for (row_index, row) in input.iter().enumerate() {
                let tree = row.get(col_index).unwrap();
                if *tree > current_height as isize {
                    visible_trees.insert((row_index, col_index));
                    current_height = *tree as i32;
                }
            }

            // from bottom
            let mut current_height: i32 = -1;
            for (row_index, row) in input.iter().enumerate().rev() {
                let tree = row.get(col_index).unwrap();
                if *tree > current_height as isize {
                    visible_trees.insert((row_index, col_index));
                    current_height = *tree as i32;
                }
            }
        }

        visible_trees.len()
    };

    let solution2 = {
        let row_len = input.len();
        let col_len = input.first().unwrap().len();

        let mut best_scenic_score = 0;

        for row_index in 1..row_len - 1 {
            for col_index in 1..col_len - 1 {
                let tree_house_height = *input.get(row_index).unwrap().get(col_index).unwrap();

                // look up
                let mut scenic_score_up = 0;
                if row_index > 0 {
                    for i in (0..=(row_index - 1)).rev() {
                        let tree = *input.get(i).unwrap().get(col_index).unwrap();
                        scenic_score_up += 1;
                        if tree >= tree_house_height {
                            break;
                        }
                    }
                }

                // look down
                let mut scenic_score_down = 0;
                for i in (row_index + 1)..row_len {
                    let tree = *input.get(i).unwrap().get(col_index).unwrap();
                    scenic_score_down += 1;
                    if tree >= tree_house_height {
                        break;
                    }
                }

                // look left
                let mut scenic_score_left = 0;
                if col_index > 0 {
                    for j in (0..=(col_index - 1)).rev() {
                        let tree = *input.get(row_index).unwrap().get(j).unwrap();
                        scenic_score_left += 1;
                        if tree >= tree_house_height {
                            break;
                        }
                    }
                }

                // look right
                let mut scenic_score_right = 0;
                for j in (col_index + 1)..col_len {
                    let tree = *input.get(row_index).unwrap().get(j).unwrap();
                    scenic_score_right += 1;
                    if tree >= tree_house_height {
                        break;
                    }
                }

                let scenic_score =
                    scenic_score_up * scenic_score_down * scenic_score_left * scenic_score_right;

                if scenic_score > best_scenic_score {
                    best_scenic_score = scenic_score;
                }
            }
        }

        best_scenic_score
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}
