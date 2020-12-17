use advent_of_code::*;
use std::collections::HashSet;
use std::hash::Hash;

trait HasNeighbors<T> {
    fn neighbors(&self) -> Vec<T>;
}

fn main() {
    let state = {
        let mut initial = HashSet::new();
        read_input_as_lines("2020/day17/src/input.txt")
            .into_iter()
            .enumerate()
            .for_each(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .for_each(|(col, _)| {
                        initial.insert(Coordinate3 {
                            x: row as isize,
                            y: col as isize,
                            z: 0,
                        });
                    })
            });
        initial
    };

    let answer1 = {
        let mut result = state.clone();
        for _ in 0..6 {
            result = perform_cycle(result);
        }
        result.len()
    };

    let answer2 = {
        let mut result = state
            .iter()
            .map(|coordinate3| (*coordinate3).into())
            .collect::<HashSet<Coordinate4>>();
        for _ in 0..6 {
            result = perform_cycle(result);
        }
        result.len()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn perform_cycle<T>(state: HashSet<T>) -> HashSet<T>
where
    T: HasNeighbors<T> + Eq + PartialEq + Hash + Copy,
{
    let mut result = HashSet::new();

    let mut inactive_neighbors = HashSet::new();
    for active_cube in state.iter() {
        let mut active_neighbor_count = 0;
        for neighbor in active_cube.neighbors() {
            match state.contains(&neighbor) {
                true => active_neighbor_count += 1,
                false => {
                    inactive_neighbors.insert(neighbor);
                }
            }
        }
        if active_neighbor_count == 2 || active_neighbor_count == 3 {
            result.insert(*active_cube);
        }
    }

    for inactive_cube in inactive_neighbors {
        let active_neighbor_count = inactive_cube
            .neighbors()
            .into_iter()
            .filter(|neighbor| state.contains(neighbor))
            .count();
        if active_neighbor_count == 3 {
            result.insert(inactive_cube);
        }
    }

    result
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate3 {
    x: isize,
    y: isize,
    z: isize,
}

impl HasNeighbors<Coordinate3> for Coordinate3 {
    fn neighbors(&self) -> Vec<Coordinate3> {
        let mut result = Vec::with_capacity(26);
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }

                    result.push(Coordinate3 {
                        x: self.x + x,
                        y: self.y + y,
                        z: self.z + z,
                    });
                }
            }
        }
        result
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl HasNeighbors<Coordinate4> for Coordinate4 {
    fn neighbors(&self) -> Vec<Coordinate4> {
        let mut result = Vec::with_capacity(80);
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if x == 0 && y == 0 && z == 0 && w == 0 {
                            continue;
                        }

                        result.push(Coordinate4 {
                            x: self.x + x,
                            y: self.y + y,
                            z: self.z + z,
                            w: self.w + w,
                        });
                    }
                }
            }
        }
        result
    }
}

impl From<Coordinate3> for Coordinate4 {
    fn from(coordinate3: Coordinate3) -> Self {
        Self {
            x: coordinate3.x,
            y: coordinate3.y,
            z: coordinate3.z,
            w: 0,
        }
    }
}
