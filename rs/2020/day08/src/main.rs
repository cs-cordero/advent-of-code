use advent_of_code::*;
use std::collections::HashSet;

fn main() {
    let instructions = read_input_as_lines("2020/day08/src/input.txt")
        .into_iter()
        .map(|line| {
            let (command, value) = split_once_from_left(&line, " ");
            (
                command.to_owned(),
                value
                    .parse::<i64>()
                    .unwrap_or_else(|_| panic!("Unable to parse value {}", value)),
            )
        })
        .collect::<Vec<_>>();

    let (_, answer1) = run_instructions(&instructions);
    let answer2 = find_termination(&instructions);

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn run_instructions(instructions: &[(String, i64)]) -> (bool, i64) {
    let mut accumulator: i64 = 0;
    let mut ip = 0;
    let mut seen_ip = HashSet::new();

    while ip < instructions.len() {
        if seen_ip.contains(&ip) {
            return (false, accumulator);
        }
        seen_ip.insert(ip);
        let (command, value) = instructions.get(ip).unwrap();
        match command.as_str() {
            "acc" => {
                accumulator += value;
                ip += 1;
            }
            "jmp" => {
                ip = (ip as i64 + value) as usize;
            }
            _ => {
                ip += 1;
            }
        }
    }
    (true, accumulator)
}

fn find_termination(instructions: &[(String, i64)]) -> i64 {
    for i in 0..instructions.len() {
        if let Some((command, value)) = instructions.get(i) {
            let new_command = match command.as_str() {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => continue,
            };

            let new_instructions = [
                &instructions[..i],
                &[(new_command.to_owned(), *value)],
                &instructions[i + 1..],
            ]
            .concat();

            let (terminated, accumulator_value) = run_instructions(&new_instructions);
            if terminated {
                return accumulator_value;
            }
        }
    }

    panic!("Did not find a way to terminate the instruction set!");
}
