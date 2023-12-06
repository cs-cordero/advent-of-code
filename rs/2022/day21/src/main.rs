extern crate core;

use advent_of_code::*;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Debug)]
enum Operation {
    Value(i64),
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

fn main() {
    let original_data = read_input_as_lines("2022/day21/src/input.txt")
        .into_iter()
        .map(|line| {
            let (monkey, spec) = line.split_once(": ").unwrap();
            let monkey = String::from(monkey);
            if let Ok(value) = spec.parse::<i64>() {
                (monkey, Operation::Value(value))
            } else {
                let mut spec = spec.split(' ');
                let (lhs, op, rhs) = (
                    spec.next().unwrap(),
                    spec.next().unwrap(),
                    spec.next().unwrap(),
                );
                let lhs = String::from(lhs);
                let rhs = String::from(rhs);
                (
                    monkey,
                    match op {
                        "+" => Operation::Add(lhs, rhs),
                        "-" => Operation::Subtract(lhs, rhs),
                        "*" => Operation::Multiply(lhs, rhs),
                        "/" => Operation::Divide(lhs, rhs),
                        _ => panic!(),
                    },
                )
            }
        })
        .collect::<HashMap<_, _>>();

    let solution1 = {
        let mut data = original_data.clone();
        dfs(&mut data, "root")
    };

    let solution2 = {
        let mut data = original_data;
        data.insert(String::from("humn"), Operation::Value(0));

        let human_chain = in_humn_chain(&data, vec!["root"], "root")
            .unwrap()
            .iter()
            .map(|s| (*s).to_string())
            .collect::<Vec<_>>();
        let non_humans = data
            .keys()
            .filter(|monkey| !human_chain.contains(monkey))
            .cloned()
            .collect::<Vec<_>>();

        for non_human in non_humans {
            dfs(&mut data, &non_human);
        }

        let mut coefficient: f64 = 1.0;
        let mut constant: f64 = 0.0;

        for monkey in human_chain.iter().skip(1).rev().skip(1) {
            match data.get(monkey).unwrap() {
                Operation::Value(_) => panic!(),
                Operation::Add(lhs, rhs) => {
                    let other = if human_chain.contains(lhs) { rhs } else { lhs };
                    if let Some(Operation::Value(value)) = data.get(other) {
                        constant += *value as f64;
                    } else {
                        panic!();
                    }
                }
                Operation::Subtract(lhs, rhs) => {
                    if human_chain.contains(lhs) {
                        if let Some(Operation::Value(value)) = data.get(rhs) {
                            constant -= *value as f64;
                        } else {
                            panic!();
                        }
                    } else if let Some(Operation::Value(value)) = data.get(lhs) {
                        constant = *value as f64 - constant;
                    } else {
                        panic!();
                    };
                }
                Operation::Multiply(lhs, rhs) => {
                    let other = if human_chain.contains(lhs) { rhs } else { lhs };
                    if let Some(Operation::Value(value)) = data.get(other) {
                        coefficient *= *value as f64;
                        constant *= *value as f64;
                    } else {
                        panic!();
                    }
                }
                Operation::Divide(lhs, rhs) => {
                    let other = if human_chain.contains(lhs) {
                        rhs
                    } else {
                        panic!()
                    };
                    if let Some(Operation::Value(value)) = data.get(other) {
                        coefficient /= *value as f64;
                        constant /= *value as f64;
                    } else {
                        panic!();
                    }
                }
            };
        }

        let (root_lhs, root_rhs) = if let Operation::Add(lhs, rhs) = data.get("root").unwrap() {
            (lhs, rhs)
        } else {
            panic!();
        };
        let other = if human_chain.contains(root_lhs) {
            root_rhs
        } else {
            root_lhs
        };

        if let Some(Operation::Value(value)) = data.get(other) {
            ((*value as f64 - constant) / coefficient).abs() as i64
        } else {
            panic!();
        }
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn dfs(graph: &mut HashMap<String, Operation>, current: &str) -> i64 {
    let operation = graph.get(current).unwrap();

    let (func, lhs, rhs): (fn(i64, i64) -> i64, String, String) = match operation {
        Operation::Value(value) => return *value,
        Operation::Add(lhs, rhs) => (i64::add, lhs.to_owned(), rhs.to_owned()),
        Operation::Subtract(lhs, rhs) => (i64::sub, lhs.to_owned(), rhs.to_owned()),
        Operation::Multiply(lhs, rhs) => (i64::mul, lhs.to_owned(), rhs.to_owned()),
        Operation::Divide(lhs, rhs) => (i64::div, lhs.to_owned(), rhs.to_owned()),
    };

    let lhs = dfs(graph, &lhs);
    let rhs = dfs(graph, &rhs);
    let result = func(lhs, rhs);

    graph.insert(current.to_owned(), Operation::Value(result));
    result
}

fn in_humn_chain<'a>(
    graph: &'a HashMap<String, Operation>,
    current: Vec<&'a str>,
    monkey: &'a str,
) -> Option<Vec<&'a str>> {
    if monkey == "humn" {
        return Some(current);
    }

    let (lhs, rhs) = match graph.get(monkey).unwrap() {
        Operation::Value(_) => return None,
        Operation::Add(lhs, rhs) => (lhs, rhs),
        Operation::Subtract(lhs, rhs) => (lhs, rhs),
        Operation::Multiply(lhs, rhs) => (lhs, rhs),
        Operation::Divide(lhs, rhs) => (lhs, rhs),
    };

    let mut copy_with_lhs = current.to_vec();
    copy_with_lhs.push(lhs);
    if let Some(result) = in_humn_chain(graph, copy_with_lhs, lhs) {
        return Some(result);
    }

    let mut copy_with_rhs = current.to_vec();
    copy_with_rhs.push(rhs);
    if let Some(result) = in_humn_chain(graph, copy_with_rhs, rhs) {
        return Some(result);
    }

    None
}
