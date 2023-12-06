use advent_of_code::*;
use std::cmp::Ordering;

fn main() {
    let data = read_input_as_lines("2021/day03/src/input.txt")
        .into_iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let answer1 = {
        let mut gamma_rate: u64 = 0;
        let mut epsilon_rate: u64 = 0;

        for index in 0..data[0].len() {
            gamma_rate <<= 1;
            epsilon_rate <<= 1;
            match compare_one_bits_to_zero_bits_at_index(&data, index) {
                Ordering::Greater | Ordering::Equal => gamma_rate += 1,
                Ordering::Less => epsilon_rate += 1,
            }
        }

        gamma_rate * epsilon_rate
    };

    let answer2 = {
        let oxygen_generator_rating = find_rating(&data, RatingType::OxygenGenerator);
        let co2_scrubber_rating = find_rating(&data, RatingType::CO2Scrubber);
        oxygen_generator_rating * co2_scrubber_rating
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn compare_one_bits_to_zero_bits_at_index(
    ratings: &[impl AsRef<[char]>],
    index: usize,
) -> Ordering {
    let one_count = ratings
        .iter()
        .filter(|rating| rating.as_ref()[index] == '1')
        .count();
    let zero_count = ratings.len() - one_count;
    one_count.cmp(&zero_count)
}

enum RatingType {
    OxygenGenerator,
    CO2Scrubber,
}

fn find_rating(ratings: &[Vec<char>], rating_type: RatingType) -> u32 {
    let mut ratings = ratings.iter().collect::<Vec<_>>();
    let total_bits = ratings[0].len();

    for index in 0..total_bits {
        // find bit criteria
        let bit_criteria = {
            match compare_one_bits_to_zero_bits_at_index(&ratings, index) {
                Ordering::Greater | Ordering::Equal => match rating_type {
                    RatingType::OxygenGenerator => '1',
                    RatingType::CO2Scrubber => '0',
                },
                Ordering::Less => match rating_type {
                    RatingType::OxygenGenerator => '0',
                    RatingType::CO2Scrubber => '1',
                },
            }
        };

        // apply bit criteria
        ratings.retain(|rating| rating[index] == bit_criteria);

        // check for success
        if ratings.len() == 1 {
            break;
        }
    }

    // join the Vec<char> to String
    let bit_string = ratings
        .first()
        .unwrap()
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("");

    u32::from_str_radix(&bit_string, 2).unwrap()
}
