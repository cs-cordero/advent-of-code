use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2021/day01/src/input.txt")
        .into_iter()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let data_windowed = data
        .windows(3)
        .map(|window| window.iter().sum::<i32>())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", count_increases(&data));
    println!("Part 2: {:?}", count_increases(&data_windowed));
}

fn count_increases(nums: &[i32]) -> usize {
    nums.windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}
