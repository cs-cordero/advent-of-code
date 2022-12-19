extern crate core;

use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use advent_of_code::*;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize
}

impl Cube {
    fn open_sides(&self, cubes: &HashSet<Cube>) -> u8 {
        let mut result: u8 = 0;

        for (dx, dy, dz) in [
            (-1, 0, 0), (1, 0, 0),
            (0, -1, 0), (0, 1, 0),
            (0, 0, -1), (0, 0, 1),
        ] {
            let adjacent_cube = Cube { x: self.x + dx, y: self.y + dy, z: self.z + dz };
            if !cubes.contains(&adjacent_cube) {
                result += 1;
            }
        }

        result
    }
}

fn main() {
    let input = read_input_as_lines("2022/day18/src/input.txt")
        .into_iter()
        .map(|line| {
            let (x, yz) = line.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
        })
        .map(|(x, y, z)| Cube { x, y, z })
        .collect::<HashSet<_>>();

    let solution1: u32 = input.iter().map(|cube| cube.open_sides(&input) as u32).sum();
    let solution2 = {
        let (min_x, min_y, min_z, max_x, max_y, max_z) = {
            let mut min_x = isize::MAX;
            let mut min_y = isize::MAX;
            let mut min_z = isize::MAX;
            let mut max_x = isize::MIN;
            let mut max_y = isize::MIN;
            let mut max_z = isize::MIN;

            for cube in input.iter() {
                min_x = min(min_x, cube.x);
                max_x = max(max_x, cube.x);
                min_y = min(min_y, cube.y);
                max_y = max(max_y, cube.y);
                min_z = min(min_z, cube.z);
                max_z = max(max_z, cube.z);
            }

            (min_x - 1, min_y - 1, min_z - 1, max_x + 1, max_y + 1, max_z + 1)
        };

        let mut queue: VecDeque<(isize, isize, isize)> = VecDeque::new();
        queue.push_back((min_x, min_y, min_z));

        let mut result = 0;
        let mut seen = HashSet::new();

        while let Some(coordinates) = queue.pop_front() {
            let (x, y, z) = coordinates;

            for (dx, dy, dz) in [
                (-1, 0, 0), (1, 0, 0),
                (0, -1, 0), (0, 1, 0),
                (0, 0, -1), (0, 0, 1),
            ] {
                let new_x = x + dx;
                let new_y = y + dy;
                let new_z = z + dz;
                let new_cube = Cube { x: new_x, y: new_y, z: new_z };

                if !(min_x..=max_x).contains(&new_x)
                    || !(min_y..=max_y).contains(&new_y)
                    || !(min_z..=max_z).contains(&new_z) {
                    continue;
                } else if input.contains(&new_cube) {
                    result += 1;
                    continue;
                } else if !seen.insert(new_cube) {
                    continue;
                }

                queue.push_back((new_x, new_y, new_z));
            }
        }

        result
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}
