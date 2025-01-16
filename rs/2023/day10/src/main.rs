use advent_of_code::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Right,
    None
}

impl Turn {
    fn derive_from(before: Direction, after: Direction) -> Self {
        match (before, after) {
            (Direction::North, Direction::West) => Turn::Left,
            (Direction::North, Direction::East) => Turn::Right,
            (Direction::North, Direction::North) => Turn::None,
            (Direction::East, Direction::North) => Turn::Left,
            (Direction::East, Direction::South) => Turn::Right,
            (Direction::East, Direction::East) => Turn::None,
            (Direction::South, Direction::East) => Turn::Left,
            (Direction::South, Direction::West) => Turn::Right,
            (Direction::South, Direction::South) => Turn::None,
            (Direction::West, Direction::South) => Turn::Left,
            (Direction::West, Direction::North) => Turn::Right,
            (Direction::West, Direction::West) => Turn::None,
            _ => panic!("oops")
        }
    }
}

struct Runner<'a> {
    graph: &'a HashMap<(usize, usize), char>,
    position: (usize, usize),
    facing: Direction
}

impl <'a> Runner<'a> {
    fn move_forward(&mut self) {
        self.position = self.get_next_position(self.position, self.facing);
    }

    fn maybe_perform_turn(&mut self) {
        let pipe = *self.graph.get(&self.position).unwrap();

        match (pipe, self.facing) {
            ('L', Direction::South) => self.facing = Direction::East,
            ('L', Direction::West) => self.facing = Direction::North,
            ('L', _) => {},

            ('J', Direction::South) => self.facing = Direction::West,
            ('J', Direction::East) => self.facing = Direction::North,
            ('J', _) => {},

            ('7', Direction::North) => self.facing = Direction::West,
            ('7', Direction::East) => self.facing = Direction::South,
            ('7', _) => {},

            ('F', Direction::North) => self.facing = Direction::East,
            ('F', Direction::West) => self.facing = Direction::South,
            ('F', _) => {},

            _ => {}
        };
    }

    fn next(&mut self) {
        self.move_forward();
        self.maybe_perform_turn();
    }

    fn peek_left(&self) -> (usize, usize) {
        self.get_next_position(self.position, self.facing.turn_left())
    }

    fn peek_right(&self) -> (usize, usize) {
        self.get_next_position(self.position, self.facing.turn_right())
    }

    fn get_next_position(&self, location: (usize, usize), direction: Direction) -> (usize, usize) {
        let (row, col) = location;
        match direction {
            Direction::North => (row - 1, col),
            Direction::South => (row + 1, col),
            Direction::East => (row, col + 1),
            Direction::West => (row, col - 1),
        }
    }
}

fn main() {
    let mut graph: HashMap<(usize, usize), char> = HashMap::new();

    let mut start_row = -1;
    let mut start_col = -1;
    for (row, line) in read_input_as_lines("2023/day10/src/input.txt").iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == 'S' {
                start_row = row as isize;
                start_col = col as isize;
            }
            graph.insert((row, col), char);
        }
    }
    assert_ne!(start_row, -1);
    assert_ne!(start_col, -1);

    let start_row = start_row as usize;
    let start_col = start_col as usize;
    let neighbors = find_neighbors(&graph, start_row, start_col);
    let start_pipe = if neighbors.contains(&Direction::North) && neighbors.contains(&Direction::South) {
        '|'
    } else if neighbors.contains(&Direction::East) && neighbors.contains(&Direction::West) {
        '-'
    } else if neighbors.contains(&Direction::West) && neighbors.contains(&Direction::North) {
        'J'
    } else if neighbors.contains(&Direction::East) && neighbors.contains(&Direction::North) {
        'L'
    } else if neighbors.contains(&Direction::West) && neighbors.contains(&Direction::South) {
        '7'
    } else if neighbors.contains(&Direction::East) && neighbors.contains(&Direction::South) {
        'F'
    } else if neighbors.is_empty() {
        '.'
    } else {
        panic!("oops");
    };
    graph.insert((start_row, start_col), start_pipe);

    let part1 = {
        let (facing1, facing2) = match start_pipe {
            '|' => (Direction::North, Direction::South),
            '-' => (Direction::West, Direction::East),
            'L' => (Direction::North, Direction::East),
            'J' => (Direction::North, Direction::West),
            'F' => (Direction::South, Direction::East),
            '7' => (Direction::South, Direction::West),
            _ => panic!("oops")
        };
        let mut runner1 = Runner {
            graph: &graph,
            position: (start_row, start_col),
            facing: facing1,
        };
        let mut runner2 = Runner {
            graph: &graph,
            position: (start_row, start_col),
            facing: facing2,
        };

        let mut steps = 1;
        runner1.next();
        runner2.next();

        while runner1.position != runner2.position {
            runner1.next();
            runner2.next();
            steps += 1;
        }

        steps
    };

    let part2 = {
        // determine which side of the pipe is enclosed
        let start = (start_row, start_col);
        let (facing, _) = match start_pipe {
            '|' => (Direction::North, Direction::South),
            '-' => (Direction::West, Direction::East),
            'L' => (Direction::North, Direction::East),
            'J' => (Direction::North, Direction::West),
            'F' => (Direction::South, Direction::East),
            '7' => (Direction::South, Direction::West),
            _ => panic!("oops")
        };
        let mut runner = Runner {
            graph: &graph,
            position: start,
            facing,
        };

        let mut main_pipe = HashSet::new();
        main_pipe.insert(start);

        let mut turn_count = 0;
        loop {
            let before_facing = runner.facing;
            runner.next();
            let after_facing = runner.facing;

            match Turn::derive_from(before_facing, after_facing) {
                Turn::Left => turn_count -= 1,
                Turn::Right => turn_count += 1,
                Turn::None => {}
            };
            main_pipe.insert(runner.position);

            if runner.position == start {
                break;
            }
        }

        let enclosed_side = if turn_count > 0 {
            Turn::Right
        } else if turn_count < 0 {
            Turn::Left
        } else {
            panic!("oops");
        };
        let peek_fn: fn(&Runner) -> (usize, usize) = match enclosed_side {
            Turn::Left => |runner| runner.peek_left(),
            Turn::Right => |runner| runner.peek_right(),
            _ => panic!("oops")
        };

        // Now collect BFS counting points while running through.
        let mut queue = VecDeque::new();
        let mut added = HashSet::new();
        loop {
            let mut candidates: Vec<(usize, usize)> = Vec::new();
            candidates.push(peek_fn(&runner));
            runner.move_forward();
            candidates.push(peek_fn(&runner));
            runner.maybe_perform_turn();
            candidates.push(peek_fn(&runner));

            for point in candidates {
                if !main_pipe.contains(&point) && !added.insert(point) {
                    queue.push_back(point);
                }
            }

            if runner.position == start {
                break;
            }
        }

        let mut answer = 0;
        let mut seen = HashSet::new();
        let mut counted = HashSet::new();
        while let Some(location) = queue.pop_front() {
            if main_pipe.contains(&location) {
                panic!("Wtf");
            }
            if !counted.insert(location) {
                continue;
            }
            answer += 1;

            let (row, col) = location;
            let adjacent_points = [
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1)
            ];
            for point in adjacent_points {
                if !seen.insert(point) || main_pipe.contains(&point) {
                    continue;
                }
                queue.push_back(point);
            }
        }

        // println!();
        // let (limit_row, limit_col) = {
        //     let mut max_row = 0;
        //     let mut max_col = 0;
        //     for (row, col) in graph.keys() {
        //         max_row = max(max_row, *row);
        //         max_col = max(max_col, *col);
        //     }
        //     (max_row, max_col)
        // };
        // for row in 0..=limit_row {
        //     let mut s = Vec::new();
        //     for col in 0..=limit_col {
        //         let coord = (row, col);
        //         if main_pipe.contains(&coord) && counted.contains(&coord) {
        //             panic!("shit");
        //         } else if main_pipe.contains(&coord) {
        //             s.push('*');
        //         } else if counted.contains(&coord) {
        //             s.push('~');
        //         } else {
        //             s.push(' ');
        //         }
        //     }
        //     let s: String = s.into_iter().collect();
        //     println!("{}", s);
        // }
        // println!();

        answer
    };

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_neighbors(graph: &HashMap<(usize, usize), char>, row: usize, col: usize) -> Vec<Direction> {
    let mut neighbors = Vec::new();

    // NORTH
    let north_pipe = graph.get(&(row - 1, col)).unwrap();
    if ['|', '7', 'F'].contains(north_pipe) {
        neighbors.push(Direction::North);
    }

    // SOUTH
    let south_pipe = graph.get(&(row + 1, col)).unwrap();
    if ['|', 'J', 'L'].contains(south_pipe) {
        neighbors.push(Direction::South);
    }

    // WEST
    let west_pipe = graph.get(&(row, col - 1)).unwrap();
    if ['-', 'L', 'F'].contains(west_pipe) {
        neighbors.push(Direction::West);
    }

    // EAST
    let east_pipe = graph.get(&(row, col + 1)).unwrap();
    if ['-', 'J', '7'].contains(east_pipe) {
        neighbors.push(Direction::East);
    }

    neighbors
}