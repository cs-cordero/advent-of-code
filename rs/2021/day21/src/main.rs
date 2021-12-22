use std::cmp::{max, min};
use std::collections::HashMap;

/// Player 1 starting position: 4
static PLAYER_1_START: u32 = 4;

/// Player 2 starting position: 3
static PLAYER_2_START: u32 = 3;
static BOARD_SIZE: u32 = 10;

fn main() {
    let answer1 = {
        let mut state = State {
            p1_position: PLAYER_1_START - 1,
            p1_score: 0,
            p2_position: PLAYER_2_START - 1,
            p2_score: 0,
        };
        let mut dice = 0;

        loop {
            brute_force_player_turn(&mut dice, &mut state.p1_score, &mut state.p1_position);
            if state.p1_score >= 1000 {
                break;
            }
            brute_force_player_turn(&mut dice, &mut state.p2_score, &mut state.p2_position);
            if state.p2_score >= 1000 {
                break;
            }
        }

        min(state.p1_score, state.p2_score) * dice
    };

    let answer2 = {
        let wins = find_wins_for_state(&State {
            p1_position: PLAYER_1_START - 1,
            p1_score: 0,
            p2_position: PLAYER_2_START - 1,
            p2_score: 0,
        });
        max(wins.0, wins.1)
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn brute_force_player_turn(dice: &mut u32, player_score: &mut u32, player_position: &mut u32) {
    *dice += 1;
    *player_position += *dice;
    *dice += 1;
    *player_position += *dice;
    *dice += 1;
    *player_position += *dice;
    *player_position %= BOARD_SIZE;
    *player_score += *player_position + 1;
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
struct State {
    p1_position: u32,
    p1_score: u32,
    p2_position: u32,
    p2_score: u32,
}

#[derive(Copy, Clone, Debug)]
struct Wins(u64, u64);

fn find_wins_for_state(state: &State) -> Wins {
    let mut memo = HashMap::new();
    find_wins_for_state_helper(state, &mut memo)
}

/// When rolling 3d3, there are 27 possible results.
///     There is 1 way to make 3: (1:1:1)
///     There are 3 ways to make 4: (1:1:2, 1:2:1, 2:1:1)
///     There are 6 ways to make 5:
///         (1:1:3, 1:3:1, 3:1:1, 1:2:2, 2:1:2, 2:2:1)
///     There are 7 ways to make 6:
///         (1:2:3, 1:3:2, 2:1:3, 2:3:1, 3:1:2, 3:2:1, 2:2:2)
///     There are 6 ways to make 7:
///         (1:3:3, 3:1:3, 3:3:1, 2:2:3, 2:3:2, 3:2:2)
///     There are 3 ways to make 8: (3:3:2, 3:2:3, 2:3:3)
///     There is 1 way to make 9: (3:3:3)
///
/// If both players get to roll without either winning, then
/// there are 27*27 possible outcomes.
fn find_wins_for_state_helper(state: &State, memo: &mut HashMap<State, Wins>) -> Wins {
    if memo.get(state).is_none() {
        let mut p1_wins = 0;
        let mut p2_wins = 0;

        let total_roll_to_ways_count = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

        for (total_roll, p1_ways) in total_roll_to_ways_count {
            let new_p1_position = (state.p1_position + total_roll) % BOARD_SIZE;
            let new_p1_score = state.p1_score + new_p1_position + 1;
            if new_p1_score >= 21 {
                p1_wins += p1_ways;
                continue;
            }

            for (total_roll, p2_ways) in total_roll_to_ways_count {
                let new_p2_position = (state.p2_position + total_roll) % BOARD_SIZE;
                let new_p2_score = state.p2_score + new_p2_position + 1;
                if new_p2_score >= 21 {
                    p2_wins += p2_ways;
                    continue;
                } else {
                    let recursive_wins = find_wins_for_state_helper(
                        &State {
                            p1_position: new_p1_position,
                            p1_score: new_p1_score,
                            p2_position: new_p2_position,
                            p2_score: new_p2_score,
                        },
                        memo,
                    );
                    p1_wins += recursive_wins.0 * p1_ways * p2_ways;
                    p2_wins += recursive_wins.1 * p1_ways * p2_ways;
                }
            }
        }

        memo.insert(*state, Wins(p1_wins, p2_wins));
    }

    *memo.get(state).unwrap()
}
