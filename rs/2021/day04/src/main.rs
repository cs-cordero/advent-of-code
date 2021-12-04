use advent_of_code::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Board {
    /// mapping of numbers on the board to their 0-based coordinate on a 5x5 grid
    number_to_coordinates: HashMap<u32, (usize, usize)>,
    /// count of seen numbers for a given row
    row_frequencies: [u32; 5],
    /// count of seen numbers for a given col
    col_frequencies: [u32; 5],
    /// keeps track of which numbers have been marked
    seen_numbers: HashSet<u32>,
}

impl Board {
    fn new(numbers: Vec<u32>) -> Self {
        assert_eq!(numbers.len(), 25);

        let number_to_coordinates = {
            let mut result = HashMap::new();
            for (i, number) in numbers.into_iter().enumerate() {
                assert!(!result.contains_key(&number), "Should not repeat numbers");

                let row = i / 5;
                let col = i % 5;
                result.insert(number, (row, col));
            }
            result
        };

        Self {
            number_to_coordinates,
            row_frequencies: [0; 5],
            col_frequencies: [0; 5],
            seen_numbers: HashSet::new(),
        }
    }

    fn is_winning(&self) -> bool {
        self.row_frequencies
            .iter()
            .chain(self.col_frequencies.iter())
            .any(|freq| *freq == 5)
    }

    fn mark_number(&mut self, number: u32) {
        if self.number_to_coordinates.contains_key(&number) && !self.seen_numbers.contains(&number)
        {
            // board has the number somewhere in its grid AND we haven't marked it yet.
            self.seen_numbers.insert(number);
            let (row, col) = self.number_to_coordinates[&number];
            self.row_frequencies[row] += 1;
            self.col_frequencies[col] += 1;
        }
    }
}

fn parse_input() -> (Vec<u32>, Vec<Board>) {
    let mut lines = read_input_as_lines("2021/day04/src/input.txt");
    let board_data = lines.split_off(1);

    // first line is the drawn numbers
    let drawn_numbers = lines
        .first()
        .unwrap()
        .trim()
        .split(',')
        .map(|unparsed| unparsed.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // create boards
    // read in 6 lines at a time, the first line of each chunk is empty.
    let mut boards: Vec<Board> = Vec::new();
    for chunk in board_data.chunks(6) {
        let numbers_in_board = chunk
            .join(" ")
            .split(' ')
            .filter(|unparsed| !unparsed.is_empty())
            .map(|unparsed| unparsed.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(numbers_in_board.len(), 25, "Each board should be 5x5");
        boards.push(Board::new(numbers_in_board));
    }

    (drawn_numbers, boards)
}

fn find_scores_for_first_and_last_winner(
    drawn_numbers: &[u32],
    mut boards: Vec<Board>,
) -> (u32, u32) {
    let mut first_winner_score: Option<u32> = None;

    for drawn_number in drawn_numbers {
        boards
            .iter_mut()
            .for_each(|board| board.mark_number(*drawn_number));

        // swap-remove winners from the boards Vec, saving the first winner and returning once
        // the last winner is found.
        // assumes there is an unambiguous first-winner and an unambiguous last-winner.
        let mut i = 0;
        while i < boards.len() {
            if boards[i].is_winning() {
                let board = boards.swap_remove(i);
                let unmarked_numbers = board
                    .number_to_coordinates
                    .keys()
                    .filter(|number| !board.seen_numbers.contains(number));
                let score = drawn_number.checked_mul(unmarked_numbers.sum()).unwrap();

                if let Some(first_winner_score) = first_winner_score {
                    if boards.is_empty() {
                        return (first_winner_score, score);
                    }
                } else {
                    first_winner_score = Some(score);
                }
            } else {
                i += 1;
            }
        }
    }

    panic!("Should have found the first and last winner");
}

fn main() {
    let (drawn_numbers, boards) = parse_input();
    let (answer1, answer2) = find_scores_for_first_and_last_winner(&drawn_numbers, boards);

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
