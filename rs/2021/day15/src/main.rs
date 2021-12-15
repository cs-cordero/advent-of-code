use advent_of_code::*;
use std::collections::BinaryHeap;

fn main() {
    let risk_levels = {
        read_input_as_string("2021/day15/src/input.txt")
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_string().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    };

    let answer1 = dijkstra(&risk_levels);

    let answer2 = {
        let expanded_risk_levels = {
            let mut expanded = risk_levels;
            let target_row_count = expanded.len() * 5;
            let target_col_count = expanded.get(0).unwrap().len() * 5;

            // expand rows
            let mut prev_row_index = 0;
            while expanded.len() < target_row_count {
                let new_row = expanded[prev_row_index]
                    .iter()
                    .map(|prev| Some(*prev + 1).filter(|value| *value < 10).unwrap_or(1))
                    .collect::<Vec<_>>();
                expanded.push(new_row);
                prev_row_index += 1;
            }

            // expand columns
            for row in expanded.iter_mut() {
                let mut prev_col_index = 0;
                while row.len() < target_col_count {
                    let new_col_value = Some(row[prev_col_index] + 1)
                        .filter(|value| *value < 10)
                        .unwrap_or(1);
                    row.push(new_col_value);
                    prev_col_index += 1;
                }
            }
            expanded
        };

        dijkstra(&expanded_risk_levels)
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn dijkstra(risk_levels: &[Vec<i32>]) -> i32 {
    let (row_limit, col_limit) = { (risk_levels.len(), risk_levels.get(0).unwrap().len()) };

    let mut dijkstra = vec![vec![i32::MAX; col_limit]; row_limit];
    let mut max_heap = BinaryHeap::new();
    max_heap.push((0, (0, 0)));

    while let Some((risk, (row, col))) = max_heap.pop() {
        let current = dijkstra[row][col];
        if -risk >= current {
            // cannot improve down this path
            continue;
        }

        dijkstra[row][col] = -risk;
        if row == row_limit - 1 && col == col_limit - 1 {
            continue;
        }

        for (next_row, next_col) in
            get_adjacent_points_manhattan((row, col), (row_limit, col_limit))
        {
            let next_risk = -risk + risk_levels[next_row][next_col];
            let known_risk = dijkstra[next_row][next_col];
            if next_risk < known_risk {
                max_heap.push((-next_risk, (next_row, next_col)));
            }
        }
    }
    *dijkstra.last().unwrap().last().unwrap()
}
