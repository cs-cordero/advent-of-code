use advent_of_code::*;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

enum Player {
    One,
    Two,
}

fn main() {
    let input = read_input_as_string("2020/day22/src/input.txt");
    let mut input = input.split("\n\n");

    let mut player_1 = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<VecDeque<_>>();

    let mut player_2 = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<VecDeque<_>>();

    let answer1 = {
        let mut player_1 = player_1.clone();
        let mut player_2 = player_2.clone();
        non_recursive_game(&mut player_1, &mut player_2);
        count_winning_hand(&player_1, &player_2)
    };

    let answer2 = {
        recursive_game(&mut player_1, &mut player_2);
        count_winning_hand(&player_1, &player_2)
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn non_recursive_game(player_1: &mut VecDeque<u32>, player_2: &mut VecDeque<u32>) -> Player {
    while !player_1.is_empty() && !player_2.is_empty() {
        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();
        if p1 > p2 {
            player_1.push_back(p1);
            player_1.push_back(p2);
        } else {
            player_2.push_back(p2);
            player_2.push_back(p1);
        }
    }

    if player_1.is_empty() {
        Player::Two
    } else {
        Player::One
    }
}

fn recursive_game(player_1: &mut VecDeque<u32>, player_2: &mut VecDeque<u32>) -> Player {
    let mut seen_p1 = HashSet::new();
    let mut seen_p2 = HashSet::new();

    while !player_1.is_empty() && !player_2.is_empty() {
        let p1_hash = hash_vecdeque(player_1);
        let p2_hash = hash_vecdeque(player_2);

        if seen_p1.contains(&p1_hash) || seen_p2.contains(&p2_hash) {
            return Player::One;
        } else {
            seen_p1.insert(p1_hash);
            seen_p2.insert(p2_hash);
        }

        let p1 = player_1.pop_front().unwrap();
        let p2 = player_2.pop_front().unwrap();

        let winner = if player_1.len() >= p1 as usize && player_2.len() >= p2 as usize {
            let mut p1_copy = player_1.clone();
            p1_copy.truncate(p1 as usize);
            let mut p2_copy = player_2.clone();
            p2_copy.truncate(p2 as usize);

            recursive_game(&mut p1_copy, &mut p2_copy)
        } else if p1 > p2 {
            Player::One
        } else {
            Player::Two
        };

        match winner {
            Player::One => {
                player_1.push_back(p1);
                player_1.push_back(p2);
            }
            Player::Two => {
                player_2.push_back(p2);
                player_2.push_back(p1);
            }
        }
    }

    if player_1.is_empty() {
        Player::Two
    } else {
        Player::One
    }
}

fn count_winning_hand(player_1: &VecDeque<u32>, player_2: &VecDeque<u32>) -> u32 {
    let winning_hand = if player_1.is_empty() {
        player_2
    } else {
        player_1
    };
    let hand_size = winning_hand.len();
    winning_hand
        .iter()
        .enumerate()
        .map(|(i, card)| (hand_size - i) as u32 * card)
        .sum::<u32>()
}

#[inline]
fn hash_vecdeque(v: &VecDeque<u32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
}
