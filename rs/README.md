# Advent of Code: Rust

These are my solutions for the [Advent of Code](https://adventofcode.com) challenges in Rust.

### Setup Instructions

#### Setup a new binary for a new day
```shell script
cd 2021
cargo new --bin --edition 2021 day01
```

#### Make the new binary accessible from the root library
```toml
[[example]]
name = "2021-01"
path = "2021/day01/src/main.rs"
```

#### Test that the new binary works
```shell script
cargo run --example 2021-01
```

You'll want to add `use advent_of_code::*` at the top of each file.
