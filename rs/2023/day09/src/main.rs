use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2023/day09/src/input.txt")
        .into_iter()
        .map(|line| line.split_ascii_whitespace()
            .map(|number| number.parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let part1 = {
        let mut result = 0;
        for sequence in &data {
            let mut sub_sequences = Vec::new();
            sub_sequences.push(sequence.clone());

            let mut current = sequence;
            loop {
                let mut next_sequence = Vec::new();
                let mut all_zeroes = true;
                for i in 1..current.len() {
                    let value = current.get(i).unwrap() - current.get(i -1).unwrap();
                    all_zeroes = all_zeroes && value == 0;
                    next_sequence.push(value);
                }
                sub_sequences.push(next_sequence);
                if all_zeroes {
                    break;
                }
                current = sub_sequences.last().unwrap();
            }

            for i in (0..(sub_sequences.len() - 1)).rev() {
                let next_subsequence_last_value = *sub_sequences.get(i + 1).unwrap().last().unwrap();
                let current_subsequence = sub_sequences.get_mut(i).unwrap();
                let last_value = *current_subsequence.last().unwrap();

                let next_value = last_value + next_subsequence_last_value;
                current_subsequence.push(next_value);
            }

            result += sub_sequences.first().unwrap().last().unwrap();
        }
        result
    };

    let part2 = {
        let mut result = 0;
        for sequence in &data {
            let mut sub_sequences = Vec::new();
            sub_sequences.push(sequence.clone());

            let mut current = sequence;
            loop {
                let mut next_sequence = Vec::new();
                let mut all_zeroes = true;
                for i in 1..current.len() {
                    let value = current.get(i).unwrap() - current.get(i -1).unwrap();
                    all_zeroes = all_zeroes && value == 0;
                    next_sequence.push(value);
                }
                sub_sequences.push(next_sequence);
                if all_zeroes {
                    break;
                }
                current = sub_sequences.last().unwrap();
            }

            for i in (0..(sub_sequences.len() - 1)).rev() {
                let next_subsequence_first_value = *sub_sequences.get(i + 1).unwrap().first().unwrap();
                let current_subsequence = sub_sequences.get_mut(i).unwrap();
                let first_value = *current_subsequence.first().unwrap();

                let prev_value = first_value - next_subsequence_first_value;
                current_subsequence.insert(0, prev_value);
            }

            result += sub_sequences.first().unwrap().first().unwrap();
        }
        result
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
