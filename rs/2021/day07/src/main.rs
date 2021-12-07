use advent_of_code::*;

fn main() {
    let data = read_input_as_string("2021/day07/src/input.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let low = *data.iter().min().unwrap();
    let high = *data.iter().max().unwrap();

    let answer1: i32 = {
        (low..=high)
            .map(|alignment| data.iter().map(|crab| (alignment - crab).abs()).sum())
            .min()
            .unwrap()
    };

    let answer2: i32 = {
        (low..=high)
            .map(|alignment| {
                data.iter()
                    .map(|crab| (alignment - crab).abs())
                    .map(|distance| (distance * (distance + 1)) / 2)
                    .sum()
            })
            .min()
            .unwrap()
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
