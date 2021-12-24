use advent_of_code::*;
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Instruction {
    on: bool,
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

#[derive(Copy, Clone, Debug)]
struct Box {
    lo_x: isize,
    lo_y: isize,
    lo_z: isize,
    hi_x: isize,
    hi_y: isize,
    hi_z: isize,
}

impl Box {
    fn size(&self) -> usize {
        (self.hi_x - self.lo_x + 1) as usize
            * (self.hi_y - self.lo_y + 1) as usize
            * (self.hi_z - self.lo_z + 1) as usize
    }

    fn is_disjoint(&self, rhs: &Box) -> bool {
        self.lo_x > rhs.hi_x
            || self.hi_x < rhs.lo_x
            || self.lo_y > rhs.hi_y
            || self.hi_y < rhs.lo_y
            || self.lo_z > rhs.hi_z
            || self.hi_z < rhs.lo_z
    }

    fn fully_contains(&self, rhs: &Box) -> bool {
        self.lo_x <= rhs.lo_x
            && self.hi_x >= rhs.hi_x
            && self.lo_y <= rhs.lo_y
            && self.hi_y >= rhs.hi_y
            && self.lo_z <= rhs.lo_z
            && self.hi_z >= rhs.hi_z
    }

    fn overlap(&self, rhs: &Box) -> Option<Box> {
        if self.is_disjoint(rhs) {
            None
        } else {
            Some(Box {
                lo_x: max(self.lo_x, rhs.lo_x),
                lo_y: max(self.lo_y, rhs.lo_y),
                lo_z: max(self.lo_z, rhs.lo_z),
                hi_x: min(self.hi_x, rhs.hi_x),
                hi_y: min(self.hi_y, rhs.hi_y),
                hi_z: min(self.hi_z, rhs.hi_z),
            })
        }
    }

    /// Given: Box[9..=11, 9..=11, 9..=11] - Box[10..=10, 9..=10, 10..=10]
    /// Expected: [
    ///     top box:    Box[9..=11, 11..=11, 9..=11]
    ///     bottom box: None
    ///     front box:  Box[9..=11, 9..=10, 11..=11]
    ///     back box:   Box[9..=11, 9..=10, 9..=9]
    ///     left box:   Box[9..=9,  9..=10, 10..=10]
    ///     right box:  Box[11..=11,  9..=10, 10..=10]
    /// ]
    fn difference(&self, rhs: &Box) -> Vec<Box> {
        if self.is_disjoint(rhs) {
            return vec![];
        }

        let rhs = self.overlap(rhs).unwrap();

        let top_box = if self.hi_y > rhs.hi_y {
            Some(Box {
                lo_x: self.lo_x,
                lo_y: rhs.hi_y + 1,
                lo_z: self.lo_z,
                hi_x: self.hi_x,
                hi_y: self.hi_y,
                hi_z: self.hi_z,
            })
        } else {
            None
        };

        let bottom_box = if self.lo_y < rhs.lo_y {
            Some(Box {
                lo_x: self.lo_x,
                lo_y: self.lo_y,
                lo_z: self.lo_z,
                hi_x: self.hi_x,
                hi_y: rhs.lo_y - 1,
                hi_z: self.hi_z,
            })
        } else {
            None
        };

        let front_box = if self.hi_z > rhs.hi_z {
            Some(Box {
                lo_x: self.lo_x,
                lo_y: bottom_box.map(|b| b.hi_y + 1).unwrap_or(self.lo_y),
                lo_z: rhs.hi_z + 1,
                hi_x: self.hi_x,
                hi_y: top_box.map(|b| b.lo_y - 1).unwrap_or(self.hi_y),
                hi_z: self.hi_z,
            })
        } else {
            None
        };

        let back_box = if self.lo_z < rhs.lo_z {
            Some(Box {
                lo_x: self.lo_x,
                lo_y: bottom_box.map(|b| b.hi_y + 1).unwrap_or(self.lo_y),
                lo_z: self.lo_z,
                hi_x: self.hi_x,
                hi_y: top_box.map(|b| b.lo_y - 1).unwrap_or(self.hi_y),
                hi_z: rhs.lo_z - 1,
            })
        } else {
            None
        };

        let left_box = if self.lo_x < rhs.lo_x {
            Some(Box {
                lo_x: self.lo_x,
                lo_y: bottom_box.map(|b| b.hi_y + 1).unwrap_or(self.lo_y),
                lo_z: back_box.map(|b| b.hi_z + 1).unwrap_or(self.lo_z),
                hi_x: rhs.lo_x - 1,
                hi_y: top_box.map(|b| b.lo_y - 1).unwrap_or(self.hi_y),
                hi_z: front_box.map(|b| b.lo_z - 1).unwrap_or(self.hi_z),
            })
        } else {
            None
        };

        let right_box = if self.hi_x > rhs.hi_x {
            Some(Box {
                lo_x: rhs.hi_x + 1,
                lo_y: bottom_box.map(|b| b.hi_y + 1).unwrap_or(self.lo_y),
                lo_z: back_box.map(|b| b.hi_z + 1).unwrap_or(self.lo_z),
                hi_x: self.hi_x,
                hi_y: top_box.map(|b| b.lo_y - 1).unwrap_or(self.hi_y),
                hi_z: front_box.map(|b| b.lo_z - 1).unwrap_or(self.hi_z),
            })
        } else {
            None
        };

        [
            top_box, bottom_box, front_box, back_box, left_box, right_box,
        ]
        .iter()
        .filter_map(|x| *x)
        .collect()
    }
}

fn main() {
    let instructions = read_input_as_string("2021/day22/src/input.txt")
        .trim()
        .lines()
        .map(|line| {
            let (on_or_off, coordinates) = line.split_once(" ").unwrap();
            let mut coordinates = coordinates.split(',');
            let x = coordinates.next().unwrap();
            let y = coordinates.next().unwrap();
            let z = coordinates.next().unwrap();

            let parse_coordinate = |s: &str| {
                let (_, range) = s.split_once("=").unwrap();
                let (low, high) = range.split_once("..").unwrap();
                let low = low.parse::<isize>().unwrap();
                let high = high.parse::<isize>().unwrap();
                min(low, high)..=max(low, high)
            };

            Instruction {
                on: on_or_off == "on",
                x: parse_coordinate(x),
                y: parse_coordinate(y),
                z: parse_coordinate(z),
            }
        })
        .collect::<Vec<_>>();

    let answer1 = {
        let mut result = HashSet::new();
        let initialization_zone = -50..=50;
        for instruction in instructions.iter() {
            if !initialization_zone.contains(instruction.x.start())
                || !initialization_zone.contains(instruction.x.end())
                || !initialization_zone.contains(instruction.y.start())
                || !initialization_zone.contains(instruction.y.end())
                || !initialization_zone.contains(instruction.z.start())
                || !initialization_zone.contains(instruction.z.end())
            {
                continue;
            }

            let turn_on = instruction.on;
            for x in instruction.x.clone() {
                for y in instruction.y.clone() {
                    for z in instruction.z.clone() {
                        let coordinate = (x, y, z);
                        if turn_on {
                            result.insert(coordinate);
                        } else {
                            result.remove(&coordinate);
                        }
                    }
                }
            }
        }
        result.len()
    };

    let answer2 = {
        let mut boxes = Vec::new();
        for instruction in instructions {
            let instruction_box = Box {
                lo_x: *instruction.x.start(),
                lo_y: *instruction.y.start(),
                lo_z: *instruction.z.start(),
                hi_x: *instruction.x.end(),
                hi_y: *instruction.y.end(),
                hi_z: *instruction.z.end(),
            };

            if instruction.on {
                turn_on_box(&mut boxes, instruction_box);
            } else {
                turn_off_box(&mut boxes, instruction_box);
            }
        }

        boxes.iter().map(|b| b.size()).sum::<usize>()
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn turn_on_box(boxes: &mut Vec<Box>, new_box: Box) {
    let mut queue = VecDeque::new();
    queue.push_back(new_box);

    while let Some(new_box) = queue.pop_front() {
        let mut is_fully_disjoint = true;
        for existing_box in boxes.iter() {
            if let Some(overlap) = existing_box.overlap(&new_box) {
                is_fully_disjoint = false;
                queue.extend(new_box.difference(&overlap));
                break;
            }
        }
        if is_fully_disjoint {
            boxes.push(new_box);
        }
    }
}

fn turn_off_box(boxes: &mut Vec<Box>, removal_box: Box) {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.extend(boxes.iter().copied());

    while let Some(existing_box) = queue.pop_front() {
        if removal_box.fully_contains(&existing_box) {
            continue;
        } else if let Some(overlap) = removal_box.overlap(&existing_box) {
            queue.extend(existing_box.difference(&overlap));
        } else {
            assert!(existing_box.is_disjoint(&removal_box));
            result.push(existing_box);
        }
    }

    *boxes = result;
}
