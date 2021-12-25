use advent_of_code::*;
use std::collections::HashSet;

#[derive(Clone, Debug)]
enum MonadOperation {
    Inp(String),
    Add(String, String),
    Mul(String, String),
    Div(String, String),
    Mod(String, String),
    Eql(String, String),
}

#[derive(Clone, Debug)]
struct Monad<'a> {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    instructions: &'a [MonadOperation],
}

impl<'a> Monad<'a> {
    fn run_with(&mut self, input: i64) -> i64 {
        self.w = input;

        assert!(matches!(
            self.instructions.iter().take(1).next().unwrap(),
            MonadOperation::Inp(_)
        ));
        for instruction in self.instructions.iter().skip(1) {
            match instruction {
                MonadOperation::Inp(_) => panic!("Each Monad should have exactly one inp command at the start of its instruction set"),
                MonadOperation::Add(lhs, rhs) => self.add(lhs, rhs),
                MonadOperation::Mul(lhs, rhs) => self.mul(lhs, rhs),
                MonadOperation::Div(lhs, rhs) => self.div(lhs, rhs),
                MonadOperation::Mod(lhs, rhs) => self.modulus(lhs, rhs),
                MonadOperation::Eql(lhs, rhs) => self.eql(lhs, rhs),
            }
        }

        self.z
    }

    fn add(&mut self, lhs: &str, rhs: &str) {
        let lhs_value = *self.get_mut_register(lhs);
        let sum = lhs_value + self.get_value(rhs);
        *self.get_mut_register(lhs) = sum;
    }

    fn mul(&mut self, lhs: &str, rhs: &str) {
        let lhs_value = *self.get_mut_register(lhs);
        let product = lhs_value * self.get_value(rhs);
        *self.get_mut_register(lhs) = product;
    }

    fn div(&mut self, lhs: &str, rhs: &str) {
        let lhs_value = *self.get_mut_register(lhs);
        let divided = lhs_value / self.get_value(rhs);
        *self.get_mut_register(lhs) = divided;
    }

    fn modulus(&mut self, lhs: &str, rhs: &str) {
        let lhs_value = *self.get_mut_register(lhs);
        let modulus = lhs_value % self.get_value(rhs);
        *self.get_mut_register(lhs) = modulus;
    }

    fn eql(&mut self, lhs: &str, rhs: &str) {
        let lhs_value = *self.get_mut_register(lhs);
        let cmp = lhs_value == self.get_value(rhs);
        *self.get_mut_register(lhs) = cmp as i64;
    }

    #[inline]
    fn get_mut_register(&mut self, register: &str) -> &mut i64 {
        match register {
            "w" => &mut self.w,
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            _ => panic!("Invalid register: {}", register),
        }
    }

    #[inline]
    fn get_value(&mut self, register: &str) -> i64 {
        if let Ok(value) = register.parse::<i64>() {
            value
        } else {
            *self.get_mut_register(register)
        }
    }
}

fn main() {
    let instructions = read_input_as_lines("2021/day24/src/input.txt")
        .into_iter()
        .map(|line| {
            let (operation, arguments) = line.split_once(" ").unwrap();
            match operation {
                "inp" => MonadOperation::Inp(arguments.to_string()),
                _ => {
                    let (lhs, rhs) = arguments.split_once(" ").unwrap();
                    let lhs = lhs.to_string();
                    let rhs = rhs.to_string();
                    match operation {
                        "add" => MonadOperation::Add(lhs, rhs),
                        "mul" => MonadOperation::Mul(lhs, rhs),
                        "div" => MonadOperation::Div(lhs, rhs),
                        "mod" => MonadOperation::Mod(lhs, rhs),
                        "eql" => MonadOperation::Eql(lhs, rhs),
                        _ => panic!("Invalid instruction"),
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let splits = instructions
        .iter()
        .enumerate()
        .filter_map(|(i, instruction)| {
            if matches!(instruction, MonadOperation::Inp(_)) {
                Some(i)
            } else {
                None
            }
        });

    let monads = {
        let mut result = Vec::new();
        let mut start = 0;
        for end in splits.skip(1) {
            result.push(Monad {
                w: 0,
                x: 0,
                y: 0,
                z: 0,
                instructions: &instructions[start..end],
            });
            start = end;
        }
        result.push(Monad {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            instructions: &instructions[start..],
        });
        result
    };

    println!("This solution takes some time to finish.  In release mode, you can expect this to take at least 100s for both parts to complete (+/- 1m)");
    let answer1 = dfs(&monads, true).unwrap();
    let answer2 = dfs(&monads, false).unwrap();

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn dfs(monads: &[Monad], highest: bool) -> Option<i64> {
    fn dfs_helper(
        monad_index: usize,
        input: i64,
        prev_z: i64,
        monads: &[Monad],
        memo: &mut HashSet<(usize, i64, i64)>,
        highest: bool,
    ) -> Option<Vec<i64>> {
        if memo.contains(&(monad_index, input, prev_z)) || monad_index >= monads.len() {
            // println!("hit a memoized value");
            None
        } else {
            memo.insert((monad_index, input, prev_z));
            let mut monad = monads.get(monad_index).unwrap().clone();
            monad.z = prev_z;
            let new_z = monad.run_with(input);

            if new_z == 0 && monad_index == monads.len() - 1 {
                Some(vec![input])
            } else {
                for next_input in get_range(highest) {
                    if let Some(mut digits_reversed) =
                        dfs_helper(monad_index + 1, next_input, monad.z, monads, memo, highest)
                    {
                        digits_reversed.push(input);
                        return Some(digits_reversed);
                    }
                }
                None
            }
        }
    }

    let mut memo = HashSet::new();
    for next_input in get_range(highest) {
        if let Some(digits_reversed) = dfs_helper(0, next_input, 0, monads, &mut memo, highest) {
            return Some(
                digits_reversed
                    .into_iter()
                    .rev()
                    .fold(0, |acc, digit| acc * 10 + digit),
            );
        }
    }
    None
}

#[inline]
fn get_range(highest: bool) -> impl Iterator<Item = i64> {
    if highest {
        (1..10).rev().collect::<Vec<_>>().into_iter()
    } else {
        (1..10).collect::<Vec<_>>().into_iter()
    }
}
