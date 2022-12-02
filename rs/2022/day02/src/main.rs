use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2022/day02/src/input.txt")
        .into_iter()
        .map(|line| {
            let (left, right) = line.split_once(' ').unwrap();
            let opponent = Choice::interpret_for_opponent(left).unwrap();
            let player = String::from(right);
            (opponent, player)
        })
        .collect::<Vec<_>>();

    let part1: i32 = data
        .iter()
        .map(|(opponent, player_string)| {
            let player = Choice::interpret_for_player_part1(player_string).unwrap();
            get_score(opponent, &player)
        })
        .sum();
    let part2: i32 = data
        .iter()
        .map(|(opponent, player_string)| {
            let player = Choice::interpret_for_player_part2(player_string, opponent).unwrap();
            get_score(opponent, &player)
        })
        .sum();

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

enum Choice {
    Rock,
    Paper,
    Scissors
}

impl Choice {
    fn interpret_for_opponent(s: &str) -> Result<Self, String> {
        match s {
            "A" => Result::Ok(Choice::Rock),
            "B" => Result::Ok(Choice::Paper),
            "C" => Result::Ok(Choice::Scissors),
            _ => Result::Err(String::from("Could not parse string into Choice"))
        }
    }

    fn interpret_for_player_part1(s: &str) -> Result<Self, String> {
        match s {
            "X" => Result::Ok(Choice::Rock),
            "Y" => Result::Ok(Choice::Paper),
            "Z" => Result::Ok(Choice::Scissors),
            _ => Result::Err(String::from("Could not parse string into Choice"))
        }
    }

    fn interpret_for_player_part2(s: &str, opponent: &Choice) -> Result<Self, String> {
        match opponent {
            Choice::Rock => match s {
                "X" => Result::Ok(Choice::Scissors),
                "Y" => Result::Ok(Choice::Rock),
                "Z" => Result::Ok(Choice::Paper),
                _ => Result::Err(String::from("Could not parse string into Choice"))
            }
            Choice::Paper => match s {
                "X" => Result::Ok(Choice::Rock),
                "Y" => Result::Ok(Choice::Paper),
                "Z" => Result::Ok(Choice::Scissors),
                _ => Result::Err(String::from("Could not parse string into Choice"))
            }
            Choice::Scissors => match s {
                "X" => Result::Ok(Choice::Paper),
                "Y" => Result::Ok(Choice::Scissors),
                "Z" => Result::Ok(Choice::Rock),
                _ => Result::Err(String::from("Could not parse string into Choice"))
            }
        }
    }
}

fn get_score(opponent: &Choice, player: &Choice) -> i32 {
    let score_for_choice = match player {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let score_for_result = match opponent {
        Choice::Rock => match player {
            Choice::Rock => 3,
            Choice::Paper => 6,
            Choice::Scissors => 0
        }
        Choice::Paper => match player {
            Choice::Rock => 0,
            Choice::Paper => 3,
            Choice::Scissors => 6
        }
        Choice::Scissors => match player {
            Choice::Rock => 6,
            Choice::Paper => 0,
            Choice::Scissors => 3
        }
    };

    score_for_choice + score_for_result
}
