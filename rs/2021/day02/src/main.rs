use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2021/day02/src/input.txt")
        .into_iter()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();
            (a.to_owned(), b.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let answer1 = {
        let mut horizontal = 0;
        let mut depth = 0;

        for (direction, value) in &data {
            match direction.as_str() {
                "forward" => horizontal += *value,
                "up" => depth -= *value,
                "down" => depth += *value,
                _ => panic!("Invalid command."),
            }
        }

        horizontal * depth
    };

    let answer2 = {
        let mut horizontal = 0;
        let mut depth = 0;
        let mut aim = 0;

        for (direction, value) in &data {
            match direction.as_str() {
                "forward" => {
                    horizontal += *value;
                    depth += aim * *value;
                }
                "up" => aim -= *value,
                "down" => aim += *value,
                _ => panic!("Invalid command."),
            }
        }

        horizontal * depth
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
