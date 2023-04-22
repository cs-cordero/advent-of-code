extern crate core;

use std::cell::RefCell;
use std::collections::HashMap;

use advent_of_code::*;

static MAX_ROW_SIZE: usize = 200;
static MAX_COL_SIZE: usize = 150;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn rotate180(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Move(usize),
    RotateLeft,
    RotateRight
}

enum CellType {
    Wall,
    Open,
}

struct Cell {
    cell_type: CellType,
    up: Option<MovementResult>,
    down: Option<MovementResult>,
    left: Option<MovementResult>,
    right: Option<MovementResult>
}

impl Cell {
    pub fn new(cell_type: CellType) -> Self {
        Cell {
            cell_type,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }
}

struct MovementResult {
    new_coordinate: (usize, usize),
    new_facing: Direction,
}

impl MovementResult {
    pub fn new(new_coordinate: (usize, usize), new_facing: Direction) -> Self {
        MovementResult { new_coordinate, new_facing }
    }
}


fn main() {
    let input  = read_input_as_string_no_trim("2022/day22/src/input.txt");
    let (raw_map, raw_instructions) = input.split_once("\n\n").unwrap();

    let instructions = parse_instructions(raw_instructions);
    let part1 = parse_into_map_part1(raw_map);
    let part2 = parse_into_map_part2(raw_map);

    println!("Part 1: {:?}", walk_and_find_score(&instructions, part1));
    println!("Part 2: {:?}", walk_and_find_score(&instructions, part2));
}

fn walk_and_find_score(instructions: &[Instruction], map: HashMap<(usize, usize), RefCell<Cell>>) -> usize {
    // find starting point
    let mut position = {
        let mut col = 0;
        while col < 150 {
            if map.contains_key(&(0, col)) {
                break;
            }
            col += 1;
        }
        (0, col)
    };
    let mut facing = Direction::Right;

    for instruction in instructions {
        match instruction {
            Instruction::RotateLeft => facing = facing.rotate_left(),
            Instruction::RotateRight => facing = facing.rotate_right(),
            Instruction::Move(count) => {
                for _ in 0..*count {
                    let current_cell = map.get(&position).unwrap().borrow();
                    let maybe_movement_result = match facing {
                        Direction::Up => current_cell.up.as_ref(),
                        Direction::Down => current_cell.down.as_ref(),
                        Direction::Left => current_cell.left.as_ref(),
                        Direction::Right => current_cell.right.as_ref(),
                    };
                    let movement_result = maybe_movement_result.unwrap();

                    let new_cell = map.get(&movement_result.new_coordinate).unwrap().borrow();
                    if matches!(new_cell.cell_type, CellType::Wall) {
                        // we're running into a wall, stop.
                        break;
                    }

                    position = movement_result.new_coordinate;
                    facing = movement_result.new_facing;
                }
            }
        }
    }

    let score_row = position.0 + 1;
    let score_col = position.1 + 1;
    let score_facing = match facing {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    1000 * score_row + 4 * score_col + score_facing
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    let mut result = Vec::new();

    let mut count = 0;
    for char in s.trim().chars() {
        if let Ok(number) = char.to_string().parse::<usize>() {
            count *= 10;
            count += number;
        } else {
            result.push(Instruction::Move(count));
            count = 0;

            match char {
                'L' => result.push(Instruction::RotateLeft),
                'R' => result.push(Instruction::RotateRight),
                _ => panic!("Oops, got {}", char)
            }
        }
    }

    if count > 0 {
        result.push(Instruction::Move(count));
    }

    result
}

fn parse_into_map_part1(input: &str) -> HashMap<(usize, usize), RefCell<Cell>> {
    let map = get_partially_filled_map(input);

    // stitch the rows left/right directions.
    for row in 0..MAX_ROW_SIZE {
        let mut left_most_coord = None;
        let mut right_most_coord = None;
        for col in 0..MAX_COL_SIZE {
            let coord = (row, col);
            if map.contains_key(&coord) {
                if left_most_coord.is_none() {
                    left_most_coord = Some(coord);
                } else {
                    right_most_coord = Some(coord);
                }
            }
        }

        let left_most_coord = left_most_coord.unwrap();
        let right_most_coord = right_most_coord.unwrap();

        let mut left_cell = map.get(&left_most_coord).unwrap().borrow_mut();
        left_cell.left = Some(MovementResult::new(right_most_coord, Direction::Left));

        let mut right_cell = map.get(&right_most_coord).unwrap().borrow_mut();
        right_cell.right = Some(MovementResult::new(left_most_coord, Direction::Right));
    }

    // stitch the columns up/down directions.
    for col in 0..MAX_COL_SIZE {
        let mut top_most_coord = None;
        let mut bottom_most_coord = None;
        for row in 0..MAX_ROW_SIZE {
            let coord = (row, col);
            if map.contains_key(&coord) {
                if top_most_coord.is_none() {
                    top_most_coord = Some(coord);
                } else {
                    bottom_most_coord = Some(coord);
                }
            }
        }

        let top_most_coord = top_most_coord.unwrap();
        let bottom_most_coord = bottom_most_coord.unwrap();

        let mut top_cell = map.get(&top_most_coord).unwrap().borrow_mut();
        top_cell.up = Some(MovementResult::new(bottom_most_coord, Direction::Up));

        let mut bottom_cell = map.get(&bottom_most_coord).unwrap().borrow_mut();
        bottom_cell.down = Some(MovementResult::new(top_most_coord, Direction::Down));
    }

    for (coord, cell) in map.iter() {
        assert!(cell.borrow().up.is_some(), "Missing up coord for {:?}", coord);
        assert!(cell.borrow().down.is_some(), "Missing down coord for {:?}", coord);
        assert!(cell.borrow().left.is_some(), "Missing left coord for {:?}", coord);
        assert!(cell.borrow().right.is_some(), "Missing right coord for {:?}", coord);
    }

    map
}

fn parse_into_map_part2(input: &str) -> HashMap<(usize, usize), RefCell<Cell>> {
    let map = get_partially_filled_map(input);

    // stitch the edges together, some of the orientations must be reversed.
    //  AB
    //  C
    // DE
    // F

    // a top goes with f left
    let seam_a_top = (50..100).map(|col| ((0, col), Direction::Up)).collect::<Vec<_>>();
    let seam_f_left = (150..200).map(|row| ((row, 0), Direction::Left)).collect::<Vec<_>>();
    let seam_af = seam_a_top.into_iter().zip(seam_f_left.into_iter());

    // a left goes with d left reversed
    let seam_a_left = (0..50).map(|row| ((row, 50), Direction::Left)).collect::<Vec<_>>();
    let seam_d_left = (100..150).map(|row| ((row, 0), Direction::Left)).rev().collect::<Vec<_>>();
    let seam_ad = seam_a_left.into_iter().zip(seam_d_left.into_iter());

    // b top goes with f bottom
    let seam_b_top = (100..150).map(|col| ((0, col), Direction::Up)).collect::<Vec<_>>();
    let seam_f_bottom = (0..50).map(|col| ((199, col), Direction::Down)).collect::<Vec<_>>();
    let seam_bf = seam_b_top.into_iter().zip(seam_f_bottom.into_iter());

    // b right goes with e right reversed
    let seam_b_right = (0..50).map(|row| ((row, 149), Direction::Right)).collect::<Vec<_>>();
    let seam_e_right = (100..150).map(|row| ((row, 99), Direction::Right)).rev().collect::<Vec<_>>();
    let seam_be = seam_b_right.into_iter().zip(seam_e_right.into_iter());

    // b bottom goes with c right
    let seam_b_bottom = (100..150).map(|col| ((49, col), Direction::Down)).collect::<Vec<_>>();
    let seam_c_right = (50..100).map(|row| ((row, 99), Direction::Right)).collect::<Vec<_>>();
    let seam_bc = seam_b_bottom.into_iter().zip(seam_c_right.into_iter());

    // c left goes with d top
    let seam_c_left = (50..100).map(|row| ((row, 50), Direction::Left)).collect::<Vec<_>>();
    let seam_d_top = (0..50).map(|col| ((100, col), Direction::Up)).collect::<Vec<_>>();
    let seam_cd = seam_c_left.into_iter().zip(seam_d_top.into_iter());

    // e bottom goes with f right
    let seam_e_bottom = (50..100).map(|col| ((149, col), Direction::Down)).collect::<Vec<_>>();
    let seam_f_right = (150..200).map(|row| ((row, 49), Direction::Right)).collect::<Vec<_>>();
    let seam_ef = seam_e_bottom.into_iter().zip(seam_f_right.into_iter());

    let all_seams = seam_af
        .chain(seam_ad)
        .chain(seam_bf)
        .chain(seam_be)
        .chain(seam_bc)
        .chain(seam_cd)
        .chain(seam_ef);

    for ((seam1_coord, seam1_direction), (seam2_coord, seam2_direction)) in all_seams {
        let mut seam1_cell = map.get(&seam1_coord)
            .unwrap_or_else(|| panic!("Failed to find {:?}", seam1_coord))
            .borrow_mut();
        match seam1_direction {
            Direction::Up => seam1_cell.up = Some(MovementResult::new(seam2_coord, seam2_direction.rotate180())),
            Direction::Down => seam1_cell.down = Some(MovementResult::new(seam2_coord, seam2_direction.rotate180())),
            Direction::Left => seam1_cell.left = Some(MovementResult::new(seam2_coord, seam2_direction.rotate180())),
            Direction::Right => seam1_cell.right = Some(MovementResult::new(seam2_coord, seam2_direction.rotate180())),
        }

        let mut seam2_cell = map.get(&seam2_coord)
            .unwrap_or_else(|| panic!("Failed to find {:?}", seam2_coord))
            .borrow_mut();
        match seam2_direction {
            Direction::Up => seam2_cell.up = Some(MovementResult::new(seam1_coord, seam1_direction.rotate180())),
            Direction::Down => seam2_cell.down = Some(MovementResult::new(seam1_coord, seam1_direction.rotate180())),
            Direction::Left => seam2_cell.left = Some(MovementResult::new(seam1_coord, seam1_direction.rotate180())),
            Direction::Right => seam2_cell.right = Some(MovementResult::new(seam1_coord, seam1_direction.rotate180())),
        }
    }

    for (coord, cell) in map.iter() {
        assert!(cell.borrow().up.is_some(), "Missing up coord for {:?}", coord);
        assert!(cell.borrow().down.is_some(), "Missing down coord for {:?}", coord);
        assert!(cell.borrow().left.is_some(), "Missing left coord for {:?}", coord);
        assert!(cell.borrow().right.is_some(), "Missing right coord for {:?}", coord);
    }

    map
}

fn get_partially_filled_map(input: &str) -> HashMap<(usize, usize), RefCell<Cell>> {
    let map = {
        let mut result = HashMap::new();
        for (row_index, row) in input.lines().enumerate() {
            for (col_index, value) in row.chars().enumerate() {
                let cell = match value {
                    '.' => Cell::new(CellType::Open),
                    '#' => Cell::new(CellType::Wall),
                    ' ' => continue,
                    _ => panic!("Got an invalid character {}.", value)
                };
                result.insert((row_index, col_index), RefCell::new(cell));
            }
        }
        result
    };

    // stitch together the middle regions
    for row_index in 0..MAX_ROW_SIZE {
        for col_index in 0..MAX_COL_SIZE {
            let maybe_cell = map.get(&(row_index, col_index));
            if maybe_cell.is_none() {
                continue;
            }

            let mut cell = maybe_cell.unwrap().borrow_mut();

            // stitch up
            if let Some(other_row) = row_index.checked_sub(1) {
                if map.get(&(other_row, col_index)).is_some() {
                    cell.up = Some(MovementResult::new((other_row, col_index), Direction::Up));
                }
            }

            // stitch right
            if col_index + 1 < MAX_COL_SIZE && map.get(&(row_index, col_index + 1)).is_some() {
                cell.right = Some(MovementResult::new((row_index, col_index + 1), Direction::Right));
            }

            // stitch down
            if row_index + 1 < MAX_ROW_SIZE && map.get(&(row_index + 1, col_index)).is_some() {
                cell.down = Some(MovementResult::new((row_index + 1, col_index), Direction::Down));
            }

            // stitch left
            if let Some(other_col) = col_index.checked_sub(1) {
                if map.get(&(row_index, other_col)).is_some() {
                    cell.left = Some(MovementResult::new((row_index, other_col), Direction::Left));
                }
            }
        }
    }

    map
}