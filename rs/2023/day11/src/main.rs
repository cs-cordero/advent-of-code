use advent_of_code::read_input_as_lines;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Cell {
    Space(isize),
    Galaxy
}

fn main() {
    let raw_data = read_input_as_lines("2023/day11/src/input.txt")
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1 = generate_answer(&raw_data, 2);
    let part2 = generate_answer(&raw_data, 1000000);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn generate_answer(raw_data: &[Vec<char>], expansion_size: isize) -> isize {
    let limit_row = raw_data.len();
    let limit_col = raw_data[0].len();

    let grid = generate_grid(&raw_data, expansion_size);

    let galaxies = {
        let mut result = HashSet::new();
        for row in 0..limit_row {
            for col in 0..limit_col {
                if grid[row][col] == Cell::Galaxy {
                    result.insert((row, col));
                }
            }
        }
        result
    };

    let mut answer = 0;

    let mut galaxies = galaxies.iter().collect::<VecDeque<_>>();
    while !galaxies.is_empty() {
        let source_galaxy = *galaxies.pop_front().unwrap();
        let costs = generate_costs(&grid, source_galaxy);
        for &&remaining_galaxy in galaxies.iter() {
            let (row, col) = remaining_galaxy;
            answer += costs[row][col];
        }
    }

    answer
}

fn generate_grid(raw_data: &[Vec<char>], expansion_size: isize) -> Vec<Vec<Cell>> {
    let limit_row = raw_data.len();
    let limit_col = raw_data[0].len();
    let mut grid = vec![vec![Cell::Space(1); limit_col]; limit_row];

    let mut expansion_rows = HashSet::new();
    (0..limit_row).for_each(|i| { expansion_rows.insert(i); });
    let mut expansion_cols = HashSet::new();
    (0..limit_col).for_each(|i| { expansion_cols.insert(i); });

    for row in 0..limit_row {
        for col in 0..limit_col {
            let raw_value = raw_data[row][col];
            if raw_value == '#' {
                grid[row][col] = Cell::Galaxy;
                expansion_rows.remove(&row);
                expansion_cols.remove(&col);
            }
        }
    }

    for expansion_row in expansion_rows {
        for col in 0..limit_col {
            grid[expansion_row][col] = Cell::Space(expansion_size);
        }
    }

    for expansion_col in expansion_cols {
        for row in 0..limit_row {
            grid[row][expansion_col] = Cell::Space(expansion_size);
        }
    }

    grid
}

fn generate_costs(grid: &Vec<Vec<Cell>>, starting_point: (usize, usize)) -> Vec<Vec<isize>> {
    let limit_col = grid[0].len();
    let limit_row = grid.len();
    let mut costs = vec![vec![isize::MAX; limit_col]; limit_row];

    let mut queue = VecDeque::new();
    queue.push_back((starting_point, 0));

    while let Some(((row, col), cost)) = queue.pop_front() {
        let cost= cost + match grid[row][col] {
            Cell::Galaxy => 1,
            Cell::Space(size) => size
        };

        if cost >= costs[row][col] {
            continue;
        }

        costs[row][col] = cost - 1;

        if let Some(next_row) = row.checked_sub(1) {
            queue.push_back(((next_row, col), cost));
        }
        if let Some(next_col) = col.checked_sub(1) {
            queue.push_back(((row, next_col), cost));
        }
        if let Some(next_row) = row.checked_add(1).filter(|&row| row < limit_row) {
            queue.push_back(((next_row, col), cost));
        }
        if let Some(next_col) = col.checked_add(1).filter(|&col| col < limit_col) {
            queue.push_back(((row, next_col), cost));
        }
    }

    costs
}
