use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let input = read_input("src/input.txt")?;
    let part1 = vec![
        vec!["1", "2", "3"],
        vec!["4", "5", "6"],
        vec!["7", "8", "9"],
    ];
    println!("Part 1: {}", solution(part1, (1, 1), &input));

    let part2 = vec![
        vec!["N", "N", "1", "N", "N"],
        vec!["N", "2", "3", "4", "N"],
        vec!["5", "6", "7", "8", "9"],
        vec!["N", "A", "B", "C", "N"],
        vec!["N", "N", "D", "N", "N"],
    ];
    println!("Part 2: {}", solution(part2, (2, 0), &input));
    return Ok(());
}

fn solution(phone: Vec<Vec<&str>>, start: (usize, usize), input: &str) -> String {
    let (mut x, mut y) = start;
    let x_max = phone[0].len();
    let y_max = phone.len();
    let mut result = String::new();

    for line in input.lines() {
        for direction in line.chars() {
            let (dx, dy, method) = get_delta(direction);
            let temp_x = match method(x, dx) {
                Some(value) => value,
                None => continue,
            };
            let temp_y = match method(y, dy) {
                Some(value) => value,
                None => continue,
            };
            if temp_x >= x_max || temp_y >= y_max {
                continue;
            }
            if phone[temp_x][temp_y] == "N" {
                continue;
            }
            x = temp_x;
            y = temp_y;
        }
        result.push_str(phone[x][y]);
    }
    return result;
}

fn get_delta(direction: char) -> (usize, usize, fn(usize, usize) -> Option<usize>) {
    return match direction {
        'U' => (1, 0, usize::checked_sub),
        'D' => (1, 0, usize::checked_add),
        'L' => (0, 1, usize::checked_sub),
        'R' => (0, 1, usize::checked_add),
        _ => panic!("Invariant!"),
    };
}

fn read_input(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}
