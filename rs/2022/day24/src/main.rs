extern crate core;

use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::*;

type Point = (isize, isize);
static TOP_LIMIT: isize = 1;
static LEFT_LIMIT: isize = 1;
static RIGHT_LIMIT: isize = 120;
static BOTTOM_LIMIT: isize = 25;
static SOURCE_COORDINATE: Point = (0, 1);
static TARGET_COORDINATE: Point = (26, 120);

#[derive(Hash, Eq, PartialEq)]
enum CellType {
    Wall,
    Blizzard(Point)
}

fn main() {
    let input  = read_input_as_lines("2022/day24/src/input.txt");
    let mut map = parse_input_into_map(&input);

    let mut iterations_cache = Vec::new();

    // precompute the next blizzard locations
    for _ in 0..1000 {
        let new_map = simulate(&map);
        iterations_cache.push(map);
        map = new_map;
    }
    iterations_cache.push(map);

    let part1 = bfs(&iterations_cache, 1, SOURCE_COORDINATE, TARGET_COORDINATE);
    let back_to_start = bfs(&iterations_cache, part1, TARGET_COORDINATE, SOURCE_COORDINATE);
    let part2 = bfs(&iterations_cache, back_to_start, SOURCE_COORDINATE, TARGET_COORDINATE);

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn bfs(
    cache: &Vec<HashMap<Point, HashSet<CellType>>>,
    starting_minute: usize,
    source: Point,
    target: Point
) -> usize {
    let mut seen = HashSet::new();

    // Rust's BinaryHeap is a max heap.
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((0, starting_minute, vec![source]));

    let mut best_minute = usize::MAX;
    while let Some((_, next_minute, path)) = priority_queue.pop() {
        let current_position = path.last().copied().unwrap();
        if current_position == target {
            let current_minute = next_minute - 1;
            if current_minute < best_minute {
                best_minute = current_minute;
            }
            continue;
        } else if !seen.insert((next_minute, current_position))
                || next_minute >= best_minute
                || cache.len() <= next_minute {
            continue;
        }

        let next_map = cache.get(next_minute).unwrap();
        let (row, col) = current_position;
        for (drow, dcol) in [(-1, 0), (0, -1), (1, 0), (0, 1), (0, 0)] {
            let next_row = row + drow;
            let next_col = col + dcol;

            let out_of_bounds = (next_row, next_col) != SOURCE_COORDINATE
                && (next_row, next_col) != target
                && (
                    next_row < TOP_LIMIT
                    || next_row > BOTTOM_LIMIT
                    || next_col < LEFT_LIMIT
                    || next_col > RIGHT_LIMIT
                );
            let next_pos_is_empty = next_map
                .get(&(next_row, next_col))
                .map(|entities| entities.is_empty())
                .unwrap_or(true);

            if !out_of_bounds && next_pos_is_empty {
                let dist = manhattan_distance(target, (next_row, next_col));
                let mut next_path = path.clone();
                next_path.push((next_row, next_col));
                priority_queue.push((-dist, next_minute + 1, next_path));
            }
        }
    }

    best_minute
}

fn manhattan_distance(target: Point, source: Point) -> isize {
    isize::abs(target.1 - source.1) + isize::abs(target.0 - source.0)
}

fn parse_input_into_map(input: &[String]) -> HashMap<Point, HashSet<CellType>> {
    let mut result = HashMap::new();
    for (row, line) in input.iter().enumerate() {
        for (col, value) in line.chars().enumerate() {
            if let Some(new_cell) = match value {
                '#' => Some(CellType::Wall),
                '>' => Some(CellType::Blizzard((0, 1))),
                'v' => Some(CellType::Blizzard((1, 0))),
                '<' => Some(CellType::Blizzard((0, -1))),
                '^' => Some(CellType::Blizzard((-1, 0))),
                _ => None
            } {
                let mut entities = HashSet::new();
                entities.insert(new_cell);
                result.insert((row as isize, col as isize), entities);
            }
        }
    }
    result
}

fn simulate(map: &HashMap<Point, HashSet<CellType>>) -> HashMap<Point, HashSet<CellType>> {
    let mut result = HashMap::new();
    for ((row, col), entities) in map.iter() {
        for entity in entities {
            match entity {
                CellType::Wall => {
                    let entry = result.entry((*row, *col)).or_insert_with(HashSet::new);
                    entry.insert(CellType::Wall);
                },
                CellType::Blizzard((drow, dcol)) => {
                    let new_row = {
                        let candidate = row + drow;
                        if candidate < TOP_LIMIT {
                            BOTTOM_LIMIT
                        } else if candidate > BOTTOM_LIMIT {
                            TOP_LIMIT
                        } else {
                            candidate
                        }
                    };
                    let new_col = {
                        let candidate = col + dcol;
                        if candidate < LEFT_LIMIT {
                            RIGHT_LIMIT
                        } else if candidate > RIGHT_LIMIT {
                            LEFT_LIMIT
                        } else {
                            candidate
                        }
                    };
                    let entry = result.entry((new_row, new_col)).or_insert_with(HashSet::new);
                    assert!(entry.insert(CellType::Blizzard((*drow, *dcol))), "{:?} already had {:?}", (new_row, new_col), (*drow, *dcol));
                }
            }
        }
    }

    result
}