use advent_of_code::*;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum FoldType {
    /// The crease is vertical so we're folding towards the left.
    Vertical,

    /// The crease is horizontal so we're folding upwards.
    Horizontal,
}

fn main() {
    let (points, folds) = {
        let raw = read_input_as_string("2021/day13/src/input.txt");
        let (raw_points, raw_folds) = raw.split_once("\n\n").unwrap();

        let points = raw_points
            .lines()
            .map(|line| {
                let (col, row) = line.split_once(',').unwrap();
                (row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
            })
            .collect::<HashSet<_>>();

        let folds = raw_folds
            .lines()
            .map(|line| {
                let line = &line["fold along ".len()..];
                let (direction, location) = line.split_once('=').unwrap();
                (
                    match direction {
                        "x" => FoldType::Vertical,
                        "y" => FoldType::Horizontal,
                        _ => unreachable!(),
                    },
                    location.parse::<usize>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        (points, folds)
    };

    let mut paper = points;
    let mut fold_iter = folds.into_iter();

    paper = perform_fold(&paper, fold_iter.next().unwrap());
    println!("Part 1: {:?}", paper.len());

    println!("Part 2:");
    for fold in fold_iter {
        paper = perform_fold(&paper, fold);
    }

    let (max_row, max_col) = {
        let mut max_row = 0;
        let mut max_col = 0;
        for (row, col) in paper.iter() {
            max_row = max(max_row, *row);
            max_col = max(max_col, *col);
        }
        (max_row, max_col)
    };

    for row in 0..=max_row {
        let repr = (0..=max_col)
            .map(|col| match paper.contains(&(row, col)) {
                true => "#",
                false => ".",
            })
            .collect::<Vec<_>>()
            .join("");
        println!("\t{}", repr);
    }
}

fn perform_fold(
    points: &HashSet<(usize, usize)>,
    (fold_type, fold_location): (FoldType, usize),
) -> HashSet<(usize, usize)> {
    points
        .iter()
        .filter(|(row, col)| match fold_type {
            FoldType::Vertical => *col != fold_location,
            FoldType::Horizontal => *row != fold_location,
        })
        .map(|(pre_row, pre_col)| {
            let row = match fold_type {
                FoldType::Vertical => *pre_row,
                FoldType::Horizontal => {
                    if *pre_row < fold_location {
                        *pre_row
                    } else {
                        fold_location - (pre_row - fold_location)
                    }
                }
            };
            let col = match fold_type {
                FoldType::Vertical => {
                    if *pre_col < fold_location {
                        *pre_col
                    } else {
                        fold_location - (pre_col - fold_location)
                    }
                }
                FoldType::Horizontal => *pre_col,
            };
            (row, col)
        })
        .collect()
}
