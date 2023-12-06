extern crate core;

use advent_of_code::*;
use std::collections::{HashSet, VecDeque};
use std::mem::MaybeUninit;

fn main() {
    let (source_position, target_position, map) = {
        let mut source = MaybeUninit::<(isize, isize)>::uninit();
        let mut target = MaybeUninit::<(isize, isize)>::uninit();
        let mut map = Vec::new();
        for (row_index, row) in read_input_as_lines("2022/day12/src/input.txt")
            .into_iter()
            .enumerate()
        {
            let mut map_row = Vec::new();
            for (col_index, height) in row.chars().enumerate() {
                if height == 'S' {
                    source.write((row_index as isize, col_index as isize));
                    map_row.push('a');
                } else if height == 'E' {
                    target.write((row_index as isize, col_index as isize));
                    map_row.push('z');
                } else {
                    map_row.push(height);
                }
            }
            map.push(map_row);
        }

        let source = unsafe { source.assume_init() };
        let target = unsafe { target.assume_init() };
        (source, target, map)
    };

    let solution1 = bfs(source_position, target_position, &map).unwrap();

    let solution2 = bfs2(target_position, &map).unwrap();

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn bfs(
    source_position: (isize, isize),
    target_position: (isize, isize),
    map: &[Vec<char>],
) -> Option<u32> {
    let limit_row = map.len() as isize;
    let limit_col = map.first().unwrap().len() as isize;

    let mut queue = VecDeque::new();
    queue.push_back((source_position, 'a', 0));

    let mut seen = HashSet::new();
    seen.insert(source_position);

    while let Some(((current_x, current_y), current_height, current_steps)) = queue.pop_front() {
        if (current_x, current_y) == target_position {
            return Some(current_steps);
        }

        for (delta_x, delta_y) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let new_x = current_x + delta_x;
            let new_y = current_y + delta_y;
            if new_x < 0 || new_y < 0 || new_x >= limit_row || new_y >= limit_col {
                continue;
            }

            let next_height = *map
                .get(new_x as usize)
                .unwrap()
                .get(new_y as usize)
                .unwrap();
            if next_height as i8 - current_height as i8 > 1 {
                continue;
            }

            if seen.contains(&(new_x, new_y)) {
                continue;
            }

            seen.insert((new_x, new_y));
            queue.push_back(((new_x, new_y), next_height, current_steps + 1));
        }
    }

    None
}

fn bfs2(begin: (isize, isize), map: &[Vec<char>]) -> Option<u32> {
    let limit_row = map.len() as isize;
    let limit_col = map.first().unwrap().len() as isize;

    let mut queue = VecDeque::new();
    queue.push_back((begin, 'z', 0));

    let mut seen = HashSet::new();
    seen.insert(begin);

    while let Some(((current_x, current_y), current_height, current_steps)) = queue.pop_front() {
        if current_height == 'a' {
            return Some(current_steps);
        }

        for (delta_x, delta_y) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let new_x = current_x + delta_x;
            let new_y = current_y + delta_y;
            if new_x < 0 || new_y < 0 || new_x >= limit_row || new_y >= limit_col {
                continue;
            }

            let next_height = *map
                .get(new_x as usize)
                .unwrap()
                .get(new_y as usize)
                .unwrap();
            if (next_height as i8 - current_height as i8) < -1 {
                continue;
            }

            if seen.contains(&(new_x, new_y)) {
                continue;
            }

            seen.insert((new_x, new_y));
            queue.push_back(((new_x, new_y), next_height, current_steps + 1));
        }
    }

    None
}
