use advent_of_code::*;

fn main() {
    let lines = read_input_as_lines("2020/day03/src/input.txt");

    let answer1 = count_trees(&lines, 3, 1);
    let answer2 = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(right, down)| count_trees(&lines, *right, *down))
        .product::<usize>();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn count_trees(grid: &[String], right: usize, down: usize) -> usize {
    grid.iter()
        .enumerate()
        .skip(down)
        .filter(|(row, line)| {
            row % down == 0 && line.chars().nth((row * right / down) % line.len()).unwrap() == '#'
        })
        .count()
}
