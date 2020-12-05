use advent_of_code::*;
use std::collections::{VecDeque, HashSet};

fn main() {
    let taken_seats = read_input_as_lines("2020/day05/src/input.txt")
        .into_iter()
        .map(|line| search_for_seat_id(&line))
        .collect::<HashSet<_>>();

    let answer1 = taken_seats.iter().max().unwrap();
    let answer2 = {
        let mut queue = VecDeque::new();
        queue.push_back((calculate_seat_id(64, 4), 1));
        queue.push_back((calculate_seat_id(64, 4), -1));

        let mut result = None;
        while !queue.is_empty() {
            let (current_id, search_direction) = queue.pop_front().unwrap();
            if !taken_seats.contains(&current_id) {
                result = Some(current_id);
                break;
            } else {
                if let Some(next_id) = match search_direction {
                    1 => current_id.checked_add(1),
                    -1 => current_id.checked_sub(1),
                    _ => None
                } {
                    queue.push_back((next_id, search_direction));
                }
            }
        }
        result.unwrap()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn search_for_seat_id(slice: &str) -> usize {
    let len = slice.len();
    let row = binary_search(0, 127, &slice[..len-3]);
    let column = binary_search(0, 7, &slice[len-3..]);
    calculate_seat_id(row, column)
}

fn binary_search(mut low: usize, mut high: usize, slice: &str) -> usize {
    let mut mid  = (low + high) / 2;
    for char in slice.chars() {
        match char {
            'F' | 'L' => high = mid,
            'B' | 'R' => low = mid + 1,
            _ => panic!("Invalid character {}", char)
        }
        mid  = (low + high) / 2;
    }
    mid
}

fn calculate_seat_id(row: usize, column: usize) -> usize {
    row * 8 + column
}
