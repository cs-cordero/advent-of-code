use advent_of_code::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let data = read_input_as_lines("2021/day08/src/input.txt")
        .iter()
        .map(|line| {
            let (clues, numbers) = line.split_once(" | ").unwrap();
            let clues = clues.split(' ').map(str::to_string).collect::<Vec<_>>();
            let numbers = numbers
                .split(' ')
                .map(|segments| {
                    let mut segments = segments.chars().collect::<Vec<_>>();
                    segments.sort_unstable();
                    segments.into_iter().collect::<String>()
                })
                .collect::<Vec<_>>();
            (clues, numbers)
        })
        .collect::<Vec<_>>();

    let answer1 = {
        let unique_counts = vec![2usize, 4, 3, 7];
        data.iter()
            .map(|(_, numbers)| {
                numbers
                    .iter()
                    .filter(|segments| unique_counts.contains(&segments.len()))
                    .count()
            })
            .sum::<usize>()
    };

    let answer2 = {
        data.iter()
            .map(|(clues, digits)| {
                let lookup = deduce_numbers(clues);
                digits
                    .iter()
                    .map(|digit| {
                        lookup
                            .get(digit)
                            .unwrap_or_else(|| panic!("failed to find {}", digit))
                    })
                    .fold(0, |acc, digit| acc * 10 + digit)
            })
            .sum::<u32>()
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn deduce_numbers(clues: &[String]) -> HashMap<String, u32> {
    // These can easily be mapped since they have a unique number of segments each.
    let one = clues
        .iter()
        .find(|clue| clue.len() == 2)
        .map(|s| make_hashset(s))
        .unwrap();
    let seven = clues
        .iter()
        .find(|clue| clue.len() == 3)
        .map(|s| make_hashset(s))
        .unwrap();
    let four = clues
        .iter()
        .find(|clue| clue.len() == 4)
        .map(|s| make_hashset(s))
        .unwrap();
    let eight = clues
        .iter()
        .find(|clue| clue.len() == 7)
        .map(|s| make_hashset(s))
        .unwrap();

    // There should be exactly 3 of each of these
    let mut clues_with_5_segments = clues
        .iter()
        .filter(|clue| clue.len() == 5)
        .map(|s| make_hashset(s))
        .collect::<Vec<_>>();
    let mut clues_with_6_segments = clues
        .iter()
        .filter(|clue| clue.len() == 6)
        .map(|s| make_hashset(s))
        .collect::<Vec<_>>();

    // We can deduce the remaining digits based on the set of segments for each number.
    let nine = {
        let position = clues_with_6_segments
            .iter()
            .position(|clue| clue.is_superset(&four))
            .unwrap();
        clues_with_6_segments.swap_remove(position)
    };
    let zero = {
        let position = clues_with_6_segments
            .iter()
            .position(|clue| clue.is_superset(&seven))
            .unwrap();
        clues_with_6_segments.swap_remove(position)
    };
    let six = clues_with_6_segments.swap_remove(0);
    assert_eq!(
        clues_with_6_segments.len(),
        0,
        "Expected to have drained the clues with length 6"
    );

    let segment_eg = eight.difference(&nine).copied().collect::<HashSet<_>>();
    let two = {
        let position = clues_with_5_segments
            .iter()
            .position(|clue| clue.is_superset(&segment_eg))
            .unwrap();
        clues_with_5_segments.swap_remove(position)
    };
    let three = {
        let position = clues_with_5_segments
            .iter()
            .position(|clue| clue.is_superset(&seven))
            .unwrap();
        clues_with_5_segments.swap_remove(position)
    };
    let five = clues_with_5_segments.swap_remove(0);
    assert_eq!(
        clues_with_5_segments.len(),
        0,
        "Expected to have drained the clues with length 5"
    );

    let mut result = HashMap::new();
    [zero, one, two, three, four, five, six, seven, eight, nine]
        .iter()
        .enumerate()
        .for_each(|(number, clue)| {
            let mut clue_characters = clue.iter().collect::<Vec<_>>();
            clue_characters.sort();
            let clue = clue_characters.into_iter().collect::<String>();
            result.insert(clue, number as u32);
        });
    result
}

fn make_hashset(s: &str) -> HashSet<char> {
    s.chars().collect()
}
