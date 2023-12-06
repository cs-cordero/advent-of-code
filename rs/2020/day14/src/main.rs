use std::collections::HashMap;

use advent_of_code::*;

#[derive(Clone, Default)]
struct Mask {
    null_mask: u64,
    one_mask: u64,
    x_mask: u64,
    floating_combinations: Vec<u64>,
}

struct AssignToAddress {
    address: u64,
    value: u64,
}

enum Command {
    SetMask(Mask),
    Assignment(AssignToAddress),
}

fn main() {
    let input_lines = read_input_as_lines("2020/day14/src/input.txt");
    let commands = process_inputs(&input_lines);

    let answer1 = {
        let mut memory = HashMap::<u64, u64>::new();
        let mut current_mask = Mask::default();
        for command in commands.iter() {
            match command {
                Command::SetMask(mask) => current_mask = mask.clone(),
                Command::Assignment(assignment) => {
                    memory.insert(
                        assignment.address,
                        assignment.value & !current_mask.null_mask | current_mask.one_mask,
                    );
                }
            }
        }
        memory.values().sum::<u64>()
    };

    let answer2 = {
        let mut memory = HashMap::<u64, u64>::new();
        let mut current_mask = Mask::default();
        for command in commands.iter() {
            match command {
                Command::SetMask(mask) => current_mask = mask.clone(),
                Command::Assignment(assignment) => {
                    let mut address = assignment.address;
                    address &= current_mask.null_mask;
                    address |= current_mask.one_mask;
                    address &= !current_mask.x_mask;
                    for combo_mask in current_mask.floating_combinations.iter() {
                        memory.insert(address ^ combo_mask, assignment.value);
                    }
                }
            }
        }
        memory.values().sum::<u64>()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn process_inputs(input: &[String]) -> Vec<Command> {
    input
        .iter()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" = ").unwrap();
            match lhs {
                "mask" => {
                    let bit_count = rhs.len();
                    let mut null_mask = 0;
                    let mut one_mask = 0;
                    let mut x_mask = 0;

                    rhs.chars().enumerate().for_each(|(i, c)| match c {
                        '0' => null_mask |= 1 << (bit_count - i - 1),
                        '1' => one_mask |= 1 << (bit_count - i - 1),
                        _ => x_mask |= 1 << (bit_count - i - 1),
                    });

                    let bit_count = rhs.len();
                    let mut floating_combinations = Vec::new();
                    for (i, c) in rhs.chars().enumerate() {
                        if c != 'X' {
                            continue;
                        }

                        let new_bit = 1 << (bit_count - i - 1);

                        let end = floating_combinations.len();
                        for i in 0..end {
                            let val = *floating_combinations.get(i).unwrap();
                            floating_combinations.push(val | new_bit);
                        }
                        floating_combinations.push(new_bit);
                    }
                    floating_combinations.push(0);

                    Command::SetMask(Mask {
                        null_mask,
                        one_mask,
                        x_mask,
                        floating_combinations,
                    })
                }
                _ => {
                    let address = (lhs[4..lhs.len() - 1]).parse().unwrap();
                    let value = rhs.parse().unwrap();
                    Command::Assignment(AssignToAddress { address, value })
                }
            }
        })
        .collect::<Vec<_>>()
}
