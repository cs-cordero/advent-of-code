use std::collections::HashMap;

fn main() {
    let input = vec![12, 1, 16, 3, 11, 0]; // Hardcoded input

    println!("Note: this solution uses a brute force. Part 2 takes a few seconds to finish.");
    println!("In release mode, it finishes in about 2-3 seconds.");
    println!("In dev mode, it takes about 40 seconds");
    println!();

    let answer1 = count_numbers(&input, 2020);
    let answer2 = count_numbers(&input, 30000000);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn count_numbers(input: &[i32], target: i32) -> i32 {
    let mut spoken_timestamps = {
        let mut h = HashMap::new();
        input.iter().enumerate().for_each(|(i, v)| {
            h.insert(*v, (-1, (i + 1) as i32));
        });
        h
    };

    let mut last_number = *input.last().unwrap();

    for turn in input.len() as i32 + 1..=target {
        last_number = {
            if let Some((most_recent_prev, last)) = spoken_timestamps.get(&last_number) {
                if *most_recent_prev == -1 {
                    0
                } else {
                    last - most_recent_prev
                }
            } else {
                panic!("Unexpected");
            }
        };

        let last = {
            let (_, result) = spoken_timestamps.entry(last_number).or_insert((-1, -1));
            *result
        };
        spoken_timestamps.insert(last_number, (last, turn as i32));
    }

    last_number
}
