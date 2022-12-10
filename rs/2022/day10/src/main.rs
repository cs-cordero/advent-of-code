extern crate core;

use advent_of_code::*;

fn main() {
    let input = read_input_as_lines("2022/day10/src/input.txt")
        .iter()
        .map(|line| {
            if line.starts_with("noop") {
                (1, 0)
            } else {
                let (_, value) = line.split_once(' ').unwrap();
                (2, value.parse::<i32>().unwrap())
            }
        })
        .collect::<Vec<_>>();

    let mut solution1: i32 = 0;
    let mut solution2 = Vec::new();

    let mut cycle = 1;
    let mut x = 1;
    for op in input {
        let (cycle_iter_count, delta) = op;

        for _ in 0..cycle_iter_count {
            handle_solution1(cycle, x, &mut solution1);
            handle_solution2(cycle, x, &mut solution2);
            cycle += 1;
        }

        x += delta;
    }

    println!("Part 1: {:?}", solution1);
    println!("Part 2:");
    for row in solution2 {
        println!("{}", row.iter().collect::<String>());
    }
}

fn handle_solution1(cycle: i32, x: i32, strength: &mut i32) {
    if cycle == 20 || (cycle > 20 && ((cycle - 20) % 40) == 0) {
        *strength += x * cycle;
    }
}

fn handle_solution2(cycle: i32, x: i32, screen: &mut Vec<Vec<char>>) {
    let crt_pos = (cycle - 1) % 40;
    if crt_pos == 0 {
        screen.push(Vec::new());
    }

    let current_row = screen.last_mut().unwrap();

    if (crt_pos - x).abs() <= 1 {
        current_row.push('#');
    } else {
        current_row.push('.');
    }
}
