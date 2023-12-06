extern crate core;

use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::*;

type Point = (isize, isize);
type Predicate = fn(&HashSet<Point>, Point) -> Option<Point>;

fn main() {
    let input = read_input_as_lines("2022/day23/src/input.txt");

    let part1 = {
        let mut elf_locations = parse_into_elf_locations(&input);
        let mut predicates = get_predicates();
        for _ in 0..10 {
            elf_locations = play_round(&elf_locations, &predicates);
            let first_check = predicates.pop_front().unwrap();
            predicates.push_back(first_check);
        }

        assert!(!elf_locations.is_empty());

        let mut min_row = isize::MAX;
        let mut max_row = isize::MIN;
        let mut min_col = isize::MAX;
        let mut max_col = isize::MIN;

        for (row, col) in &elf_locations {
            min_row = min(min_row, *row);
            max_row = max(max_row, *row);
            min_col = min(min_col, *col);
            max_col = max(max_col, *col);
        }

        let total_size = (max_row - min_row + 1) * (max_col - min_col + 1);
        total_size - elf_locations.len() as isize
    };

    let part2 = {
        let mut elf_locations = parse_into_elf_locations(&input);
        let mut predicates = get_predicates();

        let mut i = 1;
        loop {
            let new_elf_locations = play_round(&elf_locations, &predicates);
            if new_elf_locations.eq(&elf_locations) {
                break;
            }

            i += 1;

            elf_locations = new_elf_locations;
            let first_check = predicates.pop_front().unwrap();
            predicates.push_back(first_check);
        }

        i
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn get_predicates() -> VecDeque<Predicate> {
    let mut result: VecDeque<Predicate> = VecDeque::new();
    result.push_back(check_north);
    result.push_back(check_south);
    result.push_back(check_west);
    result.push_back(check_east);
    result
}

fn parse_into_elf_locations(input: &[String]) -> HashSet<Point> {
    let mut result = HashSet::new();

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, value) in row.chars().enumerate() {
            if value == '#' {
                result.insert((row_index as isize, col_index as isize));
            }
        }
    }

    result
}

fn play_round(elves: &HashSet<Point>, predicates: &VecDeque<Predicate>) -> HashSet<Point> {
    // collect proposals in a map of new_point -> original_point
    let mut proposals = HashMap::new();

    for elf in elves {
        if check_if_isolated(elves, *elf) {
            let entry = proposals.entry(*elf).or_insert_with(HashSet::new);
            assert!(entry.insert(*elf));
            continue;
        }

        let mut predicate_matched = false;
        for predicate in predicates {
            if let Some(new_position) = predicate(elves, *elf) {
                let entry = proposals.entry(new_position).or_insert_with(HashSet::new);
                assert!(entry.insert(*elf));
                predicate_matched = true;
                break;
            }
        }

        if !predicate_matched {
            let entry = proposals.entry(*elf).or_insert_with(HashSet::new);
            assert!(entry.insert(*elf));
        }
    }

    // for new_points with only one original_point, those go to the new hashset, otherwise the original_points go in.
    let mut new_elf_locations = HashSet::new();

    for (new_position, old_positions) in proposals {
        if old_positions.len() == 1 {
            assert!(new_elf_locations.insert(new_position));
        } else {
            for old_position in &old_positions {
                assert!(
                    new_elf_locations.insert(*old_position),
                    "Failed: new_position {:?}, old_positions: {:?}",
                    old_position,
                    old_positions
                );
            }
        }
    }

    new_elf_locations
}

static CHECKS_NORTH: [Point; 3] = [(-1, -1), (-1, 0), (-1, 1)];
fn check_north(elves: &HashSet<Point>, (row, col): Point) -> Option<Point> {
    for &(delta_row, delta_col) in CHECKS_NORTH.iter() {
        let test_position = ((row + delta_row), (col + delta_col));
        if elves.contains(&test_position) {
            return None;
        }
    }

    Some((row - 1, col))
}

static CHECKS_SOUTH: [Point; 3] = [(1, -1), (1, 0), (1, 1)];
fn check_south(elves: &HashSet<Point>, (row, col): Point) -> Option<Point> {
    for &(delta_row, delta_col) in CHECKS_SOUTH.iter() {
        let test_position = ((row + delta_row), (col + delta_col));
        if elves.contains(&test_position) {
            return None;
        }
    }

    Some((row + 1, col))
}

static CHECKS_WEST: [Point; 3] = [(-1, -1), (0, -1), (1, -1)];
fn check_west(elves: &HashSet<Point>, (row, col): Point) -> Option<Point> {
    for &(delta_row, delta_col) in CHECKS_WEST.iter() {
        let test_position = ((row + delta_row), (col + delta_col));
        if elves.contains(&test_position) {
            return None;
        }
    }

    Some((row, col - 1))
}

static CHECKS_EAST: [Point; 3] = [(-1, 1), (0, 1), (1, 1)];
fn check_east(elves: &HashSet<Point>, (row, col): Point) -> Option<Point> {
    for &(delta_row, delta_col) in CHECKS_EAST.iter() {
        let test_position = ((row + delta_row), (col + delta_col));
        if elves.contains(&test_position) {
            return None;
        }
    }

    Some((row, col + 1))
}

static CHECKS_ALL: [Point; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn check_if_isolated(elves: &HashSet<Point>, (row, col): Point) -> bool {
    for &(delta_row, delta_col) in CHECKS_ALL.iter() {
        let test_position = ((row + delta_row), (col + delta_col));
        if elves.contains(&test_position) {
            return false;
        }
    }

    true
}
