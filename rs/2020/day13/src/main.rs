use advent_of_code::*;
use std::cmp::Ordering;

fn main() {
    let mut input_lines = read_input_as_lines("2020/day13/src/input.txt").into_iter();
    let earliest_time = input_lines.next().unwrap().parse::<u64>().unwrap();
    let buses = input_lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i, s.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();

    let answer1 = {
        let (bus, time) = buses
            .iter()
            .map(|(_, bus_id)| (bus_id, binary_search(earliest_time, *bus_id)))
            .min_by_key(|(_, time)| *time)
            .unwrap();

        bus * (time - earliest_time)
    };

    let answer2 = {
        let mut result = 1;
        let mut mode = 1;
        for (offset, bus_id) in buses {
            while (result + offset as u64) % bus_id != 0 {
                result += mode;
            }
            mode *= bus_id;
        }
        result
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn binary_search(target: u64, bus_number: u64) -> u64 {
    let mut left = 0;
    let mut right = std::u64::MAX as u64;
    while left < right {
        let mid = (left + right) / 2;
        if let Some(time) = mid.checked_mul(bus_number) {
            match time.cmp(&target) {
                Ordering::Equal => return time,
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
            }
        } else {
            right = mid;
        }
    }
    left * bus_number
}
