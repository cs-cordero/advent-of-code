use advent_of_code::*;

fn main() {
    let part1: u32 = {
        let races = {
            let raw = read_input_as_string("2023/day06/src/input.txt");
            let (times, distances) = raw.split_once('\n').unwrap();

            times
                .split_ascii_whitespace()
                .skip(1)
                .filter(|s| !s.is_empty())
                .zip(
                    distances
                        .split_ascii_whitespace()
                        .skip(1)
                        .filter(|s| !s.is_empty()),
                )
                .map(|(time, distance)| {
                    (
                        time.parse::<u32>().unwrap(),
                        distance.parse::<u32>().unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        };

        races
            .iter()
            .map(|(time, distance)| {
                let mut victories = 0;

                for t in 0..*time {
                    let speed = t;
                    let remaining_time = time - t;
                    let travelled = speed * remaining_time;
                    if &travelled > distance {
                        victories += 1;
                    }
                }

                victories
            })
            .product()
    };

    let part2 = {
        let raw = read_input_as_string("2023/day06/src/input.txt");
        let (times, distances) = raw.split_once('\n').unwrap();

        let time = times
            .split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();
        let distance = distances
            .split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse::<i64>()
            .unwrap();

        // we can derive a quadratic formula:
        // distance_travelled = t(total_time - t)
        // 0 = -t^2 + total_time * t - distance_travelled
        //
        // x = (-b +- sqrt(b^2 - 4ac)) / 2a
        //
        // find both solutions, then find all integers between the two solutions to find the count
        // of t that wins the race.

        let a = -1f64;
        let b = time as f64;
        let c = -distance as f64;
        let sqrt_term = (b * b - 4.0 * a * c).sqrt();

        let solution1 = (-b - sqrt_term) / (2.0 * a);
        let solution2 = (-b + sqrt_term) / (2.0 * a);
        (solution1 - solution2).abs().floor() as u64
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
