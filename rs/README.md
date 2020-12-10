# Advent of Code: Rust

These are my solutions for the [Advent of Code](https://adventofcode.com) challenges in Rust.

### Setup Instructions

#### Setup a new binary for a new day
```shell script
cd 2020
cargo new --bin --edition 2018 day01
```

#### Make the new binary accessible from the root library
```toml
[[example]]
name = "2020-01"
path = "2020/day01/src/main.rs"
```

#### Test that the new binary works
```shell script
cargo run --example 2020-01
```

You'll want to add `use advent_of_code::*` at the top of each file.
let answer2 = {
let size = input.len();
let mut dp = {
let mut v = vec![0_i64; size];
*v.last_mut().unwrap() = 1;
v
};

        for (index, value) in input.iter().enumerate().rev().skip(1) {
            println!("handlling {}", index);
            let bound = min(index + 3, size);

            let foo = ((index + 1)..bound)
                .filter_map(|other_index| {
                    input
                        .get(other_index)
                        .zip(dp.get(other_index))
                        .filter(|(other_value, _)| *other_value - value <= 3)
                        .map(|(_, dp_value)| dp_value)
                })
