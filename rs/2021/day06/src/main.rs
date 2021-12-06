use advent_of_code::*;

fn main() {
    let data = read_input_as_string("2021/day06/src/input.txt")
        .trim()
        .split(',')
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let memoized_counts = child_counts();
    let answer1 = find_fish_count(&data, &memoized_counts, 80);
    let answer2 = find_fish_count(&data, &memoized_counts, 256);

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn find_fish_count(initial_fish: &[u32], memoized_counts: &[u64; 300], days_elapsed: u32) -> u64 {
    let recursive_children_count = initial_fish
        .iter()
        .map(|first_cycle| {
            let days_remaining = (days_elapsed - 1)
                .checked_sub(*first_cycle)
                .map_or(0, |days| days + 9);
            memoized_counts[days_remaining as usize]
        })
        .sum::<u64>();

    initial_fish.len() as u64 + recursive_children_count
}

fn child_counts() -> [u64; 300] {
    let mut memo = [0u64; 300];

    for days_remaining in 9..300 {
        let immediate_children = (days_remaining as u64 - 9) / 7 + 1;

        let days_remaining_after_first_cycle = days_remaining - 9;
        let recursive_grandchildren = (0..=days_remaining_after_first_cycle)
            .rev()
            .step_by(7)
            .map(|days| memo[days])
            .sum::<u64>();

        memo[days_remaining] = immediate_children + recursive_grandchildren;
    }

    memo
}
