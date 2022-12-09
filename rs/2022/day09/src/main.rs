extern crate core;

use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::Deref;
use advent_of_code::*;

struct RopeNode {
    x: isize,
    y: isize,
}

impl RopeNode {
    fn react_to_other_node(&mut self, other: &RopeNode) {
        let distance_x = other.x - self.x;
        let distance_y = other.y - self.y;

        if distance_x != 0 && distance_y != 0 && (distance_x.abs() + distance_y.abs() > 2) {
            self.x += distance_x.signum();
            self.y += distance_y.signum();
        } else if distance_x.abs() > 1 {
            self.x += distance_x.signum();
        } else if distance_y.abs() > 1 {
            self.y += distance_y.signum();
        }
    }
}

fn main() {
    let input = read_input_as_lines("2022/day09/src/input.txt")
        .iter()
        .map(|line| {
            let (direction, count) = line.split_once(' ').unwrap();
            let direction = direction.chars().into_iter().next().unwrap();
            let count = count.parse::<u32>().unwrap();
            (direction, count)
        })
        .collect::<Vec<_>>();

    let solution1 = {
        let rope = vec![
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 })
        ];

        solve(rope, &input)
    };

    let solution2 = {
        let rope = vec![
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 }),
            RefCell::new(RopeNode { x: 0, y: 0 })
        ];

        solve(rope, &input)
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn solve(rope: Vec<RefCell<RopeNode>>, input: &[(char, u32)]) -> usize {
    let mut visited_locations = HashSet::new();

    for (direction, count) in input.iter() {
        for _ in 0..*count {
            match direction {
                'U' => rope.get(0).unwrap().borrow_mut().x -= 1,
                'D' => rope.get(0).unwrap().borrow_mut().x += 1,
                'L' => rope.get(0).unwrap().borrow_mut().y -= 1,
                'R' => rope.get(0).unwrap().borrow_mut().y += 1,
                _ => panic!("oops")
            };

            rope_reacts(&rope);

            let tail = rope.last().unwrap().borrow();
            visited_locations.insert((tail.x, tail.y));
        }
    }

    visited_locations.len()
}

fn rope_reacts(nodes: &[RefCell<RopeNode>]) {
    for i in 1..nodes.len() {
        let prev_node = nodes.get(i - 1).unwrap().borrow();
        let mut curr_node = nodes.get(i).unwrap().borrow_mut();

        curr_node.react_to_other_node(prev_node.deref());
    }
}
