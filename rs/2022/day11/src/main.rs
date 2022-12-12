extern crate core;

use std::collections::VecDeque;
use advent_of_code::*;

fn main() {
    let monkeys: Vec<Monkey> = read_input_as_string("2022/day11/src/input.txt")
        .split("\n\n")
        .map(Monkey::parse_input)
        .collect::<Vec<Monkey>>();

    let solution1 = {
        let mut monkeys = monkeys.to_vec();

        for _ in 0..20 {
            play_round(&mut monkeys, |new| new / 3);
        }

        monkeys.sort_by_key(|monkey| monkey.inspections);
        monkeys.iter().rev().take(2).map(|monkey| monkey.inspections).product::<u64>()
    };

    let solution2 = {
        let mut monkeys = monkeys.to_vec();

        let divisor = monkeys.iter()
            .map(|monkey| monkey.test.divisible_by)
            .product::<u64>();

        let adjustment = move |new| new % divisor;

        for _ in 0..10000 {
            play_round(&mut monkeys, adjustment);
        }

        monkeys.sort_by_key(|monkey| monkey.inspections);
        monkeys.iter().rev().take(2).map(|monkey| monkey.inspections).product::<u64>()
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

#[derive(Clone)]
enum Operation {
    Add(u64),
    Mul(u64),
    Squared
}

#[derive(Clone)]
struct Test {
    divisible_by: u64,
    on_success: usize,
    on_failure: usize
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: Test,
    inspections: u64
}

impl Monkey {
    fn parse_input(s: &str) -> Monkey {
        let mut lines = s.lines();
        lines.next();

        let items = {
            let line = lines.next().unwrap();
            let (_, data) = line.split_once("items: ").unwrap();
            data
                .split(", ")
                .map(|value| value.parse::<u64>().unwrap())
                .collect::<VecDeque<_>>()
        };

        let op_line = lines.next().unwrap();
        let (_, unparsed) = op_line.split_once("Operation: new = old ").unwrap();
        let operation = {
            if unparsed == "* old" {
                Operation::Squared
            } else {
                let (op, value) = unparsed.split_once(' ').unwrap();
                let value = value.parse::<u64>().unwrap();
                if op == "*" {
                    Operation::Mul(value)
                } else if op == "+" {
                    Operation::Add(value)
                } else {
                    panic!("oops");
                }
            }
        };

        let test_line = lines.next().unwrap();
        let (_, test_value) = test_line.split_once("divisible by ").unwrap();
        let test_value = test_value.parse::<u64>().unwrap();

        let success_line = lines.next().unwrap();
        let (_, success_value) = success_line.split_once("throw to monkey ").unwrap();
        let success_value = success_value.parse::<usize>().unwrap();

        let fail_line = lines.next().unwrap();
        let (_, fail_value) = fail_line.split_once("throw to monkey ").unwrap();
        let fail_value = fail_value.parse::<usize>().unwrap();

        let test = Test {
            divisible_by: test_value,
            on_success: success_value,
            on_failure: fail_value
        };

        Monkey {
            items,
            operation,
            test,
            inspections: 0
        }
    }
}

fn play_round<F: Fn(u64) -> u64>(monkeys: &mut Vec<Monkey>, adjustment: F) {
    for i in 0..monkeys.len() {
        let mut current_monkey = monkeys.get_mut(i).unwrap();

        let mut temp = Vec::new();

        while let Some(item) = current_monkey.items.pop_front() {
            current_monkey.inspections += 1;

            let new = match current_monkey.operation {
                Operation::Add(value) => item + value,
                Operation::Mul(value) => item * value,
                Operation::Squared => item * item
            };

            let new = adjustment(new);

            let new_index = {
                if new % current_monkey.test.divisible_by == 0 {
                    current_monkey.test.on_success
                } else {
                    current_monkey.test.on_failure
                }
            };

            temp.push((new, new_index));
        }

        for (value, index) in temp {
            monkeys.get_mut(index).unwrap().items.push_back(value);
        }
    }
}
