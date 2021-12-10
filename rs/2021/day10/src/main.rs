use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2021/day10/src/input.txt")
        .into_iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut answer1 = 0;
    let mut autocomplete_scores = Vec::with_capacity(data.len());

    for line in data {
        let mut stack = Vec::with_capacity(line.len());

        let mut is_valid = true;
        for bracket in line {
            is_valid = match bracket {
                '(' | '[' | '{' | '<' => {
                    stack.push(bracket);
                    true
                }
                ')' => stack.pop().map(|it| it == '(').unwrap_or(false),
                ']' => stack.pop().map(|it| it == '[').unwrap_or(false),
                '}' => stack.pop().map(|it| it == '{').unwrap_or(false),
                '>' => stack.pop().map(|it| it == '<').unwrap_or(false),
                _ => false,
            };

            if !is_valid {
                answer1 += match bracket {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => unreachable!(),
                };
            }
        }

        if is_valid {
            let mut score: u64 = 0;
            for unpaired_bracket in stack.iter().rev() {
                let points = match *unpaired_bracket {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };

                score *= 5;
                score += points
            }
            autocomplete_scores.push(score);
        }
    }

    autocomplete_scores.sort_unstable();
    let answer2 = autocomplete_scores[autocomplete_scores.len() / 2];

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
