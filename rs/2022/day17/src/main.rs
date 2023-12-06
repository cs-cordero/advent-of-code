extern crate core;

use advent_of_code::*;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

struct Block {
    points: [Point; 5],
}

impl Block {
    fn make_next_block(block_count: u64, bottom_left: Point) -> Self {
        match block_count % 5 {
            0 => Block::make_horizontal(bottom_left),
            1 => Block::make_cross(bottom_left),
            2 => Block::make_backward_l(bottom_left),
            3 => Block::make_vertical(bottom_left),
            4 => Block::make_chunk(bottom_left),
            _ => unreachable!(),
        }
    }

    fn make_horizontal(bottom_left: Point) -> Self {
        let p1 = bottom_left;
        let p2 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y,
        };
        let p3 = Point {
            x: bottom_left.x + 2,
            y: bottom_left.y,
        };
        let p4 = Point {
            x: bottom_left.x + 3,
            y: bottom_left.y,
        };
        Self {
            points: [p1, p2, p3, p4, p1],
        }
    }

    fn make_cross(bottom_left: Point) -> Self {
        let p1 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y,
        };
        let p2 = Point {
            x: bottom_left.x,
            y: bottom_left.y + 1,
        };
        let p3 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y + 1,
        };
        let p4 = Point {
            x: bottom_left.x + 2,
            y: bottom_left.y + 1,
        };
        let p5 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y + 2,
        };
        Self {
            points: [p1, p2, p3, p4, p5],
        }
    }

    fn make_backward_l(bottom_left: Point) -> Self {
        let p1 = Point {
            x: bottom_left.x,
            y: bottom_left.y,
        };
        let p2 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y,
        };
        let p3 = Point {
            x: bottom_left.x + 2,
            y: bottom_left.y,
        };
        let p4 = Point {
            x: bottom_left.x + 2,
            y: bottom_left.y + 1,
        };
        let p5 = Point {
            x: bottom_left.x + 2,
            y: bottom_left.y + 2,
        };
        Self {
            points: [p1, p2, p3, p4, p5],
        }
    }

    fn make_vertical(bottom_left: Point) -> Self {
        let p1 = Point {
            x: bottom_left.x,
            y: bottom_left.y,
        };
        let p2 = Point {
            x: bottom_left.x,
            y: bottom_left.y + 1,
        };
        let p3 = Point {
            x: bottom_left.x,
            y: bottom_left.y + 2,
        };
        let p4 = Point {
            x: bottom_left.x,
            y: bottom_left.y + 3,
        };
        Self {
            points: [p1, p2, p3, p4, p1],
        }
    }

    fn make_chunk(bottom_left: Point) -> Self {
        let p1 = Point {
            x: bottom_left.x,
            y: bottom_left.y,
        };
        let p2 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y,
        };
        let p3 = Point {
            x: bottom_left.x,
            y: bottom_left.y + 1,
        };
        let p4 = Point {
            x: bottom_left.x + 1,
            y: bottom_left.y + 1,
        };
        Self {
            points: [p1, p2, p3, p4, p1],
        }
    }

    fn move_left(&mut self, blockers: &HashSet<Point>) -> bool {
        let mut new_points = self.points;
        for new_point in &mut new_points {
            if let Some(new_x) = new_point.x.checked_sub(1) {
                new_point.x = new_x;
            } else {
                return false;
            }

            if blockers.contains(new_point) {
                return false;
            }
        }

        self.points = new_points;
        true
    }

    fn move_right(&mut self, blockers: &HashSet<Point>) -> bool {
        let mut new_points = self.points;
        for new_point in &mut new_points {
            new_point.x += 1;

            if new_point.x >= 7 || blockers.contains(new_point) {
                return false;
            }
        }

        self.points = new_points;
        true
    }

    fn move_down(&mut self, blockers: &HashSet<Point>) -> bool {
        let mut new_points = self.points;

        for new_point in &mut new_points {
            if let Some(new_y) = new_point.y.checked_sub(1) {
                new_point.y = new_y;
            } else {
                return false;
            }

            if blockers.contains(new_point) {
                return false;
            }
        }

        self.points = new_points;
        true
    }
}

fn main() {
    let input = read_input_as_string("2022/day17/src/input.txt");

    let solution1 = part1(input.chars().cycle());
    let solution2 = part2(input.chars().cycle());

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn part1(mut inputs: impl Iterator<Item = char>) -> usize {
    let mut settled_blocks: HashSet<Point> = HashSet::new();

    for block_count in 0..2022 {
        run_the_block(&mut inputs, &mut settled_blocks, block_count);
    }

    settled_blocks.iter().map(|point| point.y).max().unwrap() + 1
}

struct CycleDetails {
    start_block_count: usize,
    start_height: usize,
    height_modulo_from_start: Vec<usize>,
}

impl CycleDetails {
    fn predict_height_at_block_count(&self, block_count: usize) -> usize {
        assert!(block_count >= self.start_block_count);
        let cycle_length = self.height_modulo_from_start.len();

        let mut height = self.start_height;

        // add the full cycle height change
        let total_cycles_since_start = (block_count - self.start_block_count) / cycle_length;
        height += total_cycles_since_start * self.height_modulo_from_start.first().unwrap();

        // add any differential from the middle of the cycle.
        let remainder_block_count = (block_count - self.start_block_count) % cycle_length;
        if remainder_block_count > 1 {
            height += self
                .height_modulo_from_start
                .get(remainder_block_count)
                .unwrap();
        }
        height + 1
    }
}

fn part2(mut inputs: impl Iterator<Item = char>) -> usize {
    let snapshot_height = 50;

    let mut snapshots: HashSet<String> = HashSet::new();

    let mut settled_blocks: HashSet<Point> = HashSet::new();
    let mut block_count = 0;

    // locate a start of the cycle.
    let cycle_start = loop {
        run_the_block(&mut inputs, &mut settled_blocks, block_count);
        block_count += 1;

        let snapshot = create_snapshot(&settled_blocks, snapshot_height);
        if !snapshots.insert(snapshot.clone()) {
            let max_y = settled_blocks
                .iter()
                .map(|point| point.y)
                .max()
                .unwrap_or(0);
            break (block_count, snapshot, max_y);
        }
    };

    // collect cycle data until we reach the start again.
    let mut cycle_data = Vec::new();
    loop {
        run_the_block(&mut inputs, &mut settled_blocks, block_count);
        block_count += 1;

        let snapshot = create_snapshot(&settled_blocks, snapshot_height);
        assert!(snapshots.contains(&snapshot));

        let max_y = settled_blocks
            .iter()
            .map(|point| point.y)
            .max()
            .unwrap_or(0);
        if snapshot == cycle_start.1 {
            cycle_data.insert(0, (block_count, max_y));
            break;
        } else {
            cycle_data.push((block_count, max_y));
        }
    }

    // now that we have the cycle, we can predict the height at any block_count.
    let cycle = CycleDetails {
        start_block_count: cycle_start.0 as usize,
        start_height: cycle_start.2,
        height_modulo_from_start: cycle_data
            .into_iter()
            .map(|(_, y)| y - cycle_start.2)
            .collect(),
    };

    cycle.predict_height_at_block_count(1000000000000)
}

fn run_the_block(
    inputs: &mut impl Iterator<Item = char>,
    settled_blocks: &mut HashSet<Point>,
    current_block_number: u64,
) -> usize {
    let spawn_x = 2;
    let spawn_y = settled_blocks
        .iter()
        .map(|point| point.y)
        .max()
        .map(|y| y + 4)
        .unwrap_or(3);

    let mut block = Block::make_next_block(
        current_block_number,
        Point {
            x: spawn_x,
            y: spawn_y,
        },
    );

    loop {
        let next_command = inputs.next().unwrap();
        match next_command {
            '>' => {
                block.move_right(settled_blocks);
            }
            '<' => {
                block.move_left(settled_blocks);
            }
            _ => unreachable!(),
        }

        if !block.move_down(settled_blocks) {
            settled_blocks.extend(block.points);
            break;
        }
    }

    settled_blocks.iter().map(|point| point.y).max().unwrap() + 1
}

fn create_snapshot(settled_blocks: &HashSet<Point>, snapshot_height: usize) -> String {
    let max_y = settled_blocks
        .iter()
        .map(|point| point.y)
        .max()
        .unwrap_or(0);
    let range = (max_y.saturating_sub(snapshot_height)..=max_y).rev();

    let mut chars = Vec::new();
    for y in range {
        for x in 0..7 {
            if settled_blocks.contains(&Point { x, y }) {
                chars.push('#');
            } else {
                chars.push('.');
            }
        }
    }

    chars.into_iter().collect()
}
