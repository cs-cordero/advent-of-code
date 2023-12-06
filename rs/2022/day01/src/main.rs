use advent_of_code::*;

fn main() {
    let mut data = read_input_as_string("2022/day01/src/input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|calories| calories.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    data.sort();
    data.reverse();

    println!("Part 1: {:?}", data.first().unwrap());
    println!("Part 2: {:?}", data.iter().take(3).sum::<i32>());
}
