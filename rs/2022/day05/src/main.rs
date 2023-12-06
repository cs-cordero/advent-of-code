use advent_of_code::*;
use std::iter::FromIterator;

fn main() {
    let input = read_input_as_string("2022/day05/src/input.txt");
    let (stacks_raw, moves_raw) = input.split_once("\n\n").unwrap();

    let solution1 = part1(get_stacks(stacks_raw), &get_moves(moves_raw));
    let solution2 = part2(get_stacks(stacks_raw), &get_moves(moves_raw));

    println!("Part 1: {}", solution1);
    println!("Part 2: {}", solution2);
}

fn get_stacks(stacks_raw: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    for line in stacks_raw.lines().rev().skip(1) {
        let chars: Vec<char> = line.chars().collect();
        push_to_stack_from_index(stacks.get_mut(0).unwrap(), chars.get(1));
        push_to_stack_from_index(stacks.get_mut(1).unwrap(), chars.get(5));
        push_to_stack_from_index(stacks.get_mut(2).unwrap(), chars.get(9));
        push_to_stack_from_index(stacks.get_mut(3).unwrap(), chars.get(13));
        push_to_stack_from_index(stacks.get_mut(4).unwrap(), chars.get(17));
        push_to_stack_from_index(stacks.get_mut(5).unwrap(), chars.get(21));
        push_to_stack_from_index(stacks.get_mut(6).unwrap(), chars.get(25));
        push_to_stack_from_index(stacks.get_mut(7).unwrap(), chars.get(29));
        push_to_stack_from_index(stacks.get_mut(8).unwrap(), chars.get(33));
    }

    stacks
}

fn get_moves(moves_raw: &str) -> Vec<(usize, usize, usize)> {
    moves_raw
        .lines()
        .map(|line| {
            let (count, rest) = line.split_once(" from ").unwrap();
            let (_, count) = count.split_once(' ').unwrap();
            let (source_index, target_index) = rest.split_once(" to ").unwrap();

            let count = count.parse::<usize>().unwrap();
            let source_index = source_index.parse::<usize>().unwrap() - 1;
            let target_index = target_index.parse::<usize>().unwrap() - 1;

            (count, source_index, target_index)
        })
        .collect::<Vec<_>>()
}

fn push_to_stack_from_index(stack: &mut Vec<char>, value: Option<&char>) {
    if let Some(item) = value {
        if !item.is_whitespace() {
            stack.push(*item);
        }
    }
}

fn part1(mut stacks: Vec<Vec<char>>, moves: &[(usize, usize, usize)]) -> String {
    for (count, source_index, target_index) in moves {
        for _ in 0..*count {
            let moved_char = stacks.get_mut(*source_index).unwrap().pop().unwrap();
            stacks.get_mut(*target_index).unwrap().push(moved_char);
        }
    }

    String::from_iter(stacks.iter_mut().map(|stack| stack.pop().unwrap()))
}

fn part2(mut stacks: Vec<Vec<char>>, moves: &[(usize, usize, usize)]) -> String {
    for (count, source_index, target_index) in moves {
        let mut temp = Vec::with_capacity(*count);

        for _ in 0..*count {
            let moved_char = stacks.get_mut(*source_index).unwrap().pop().unwrap();
            temp.push(moved_char);
        }

        while let Some(element) = temp.pop() {
            stacks.get_mut(*target_index).unwrap().push(element);
        }
    }

    String::from_iter(stacks.iter_mut().map(|stack| stack.pop().unwrap()))
}
