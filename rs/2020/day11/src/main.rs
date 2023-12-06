use std::cmp::Ordering;

use advent_of_code::*;

struct Grid {
    grid: Vec<char>,
    stride: usize,
    size: usize,
}

impl Grid {
    fn new() -> Self {
        let input_lines = read_input_as_lines("2020/day11/src/input.txt");
        let stride = input_lines.first().unwrap().len();

        let grid = input_lines
            .into_iter()
            .flat_map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let size = grid.len();

        Grid { grid, stride, size }
    }

    fn play_once<F: Fn(&[char]) -> bool>(
        &mut self,
        is_neighbor_taken: F,
        required_neighbor_count: usize,
    ) -> Option<usize> {
        let mut changed = false;

        let previous = self.grid.clone();
        let neighbor_counts = {
            (0..self.size)
                .map(|i| {
                    let ray = self.cast_rays(i);
                    ray.into_iter().filter(|ray| is_neighbor_taken(ray)).count()
                })
                .collect::<Vec<_>>()
        };

        for ((current_seat, prev_seat), neighbor_count) in self
            .grid
            .iter_mut()
            .zip(previous)
            .zip(neighbor_counts.into_iter())
        {
            if prev_seat == 'L' && neighbor_count == 0 {
                changed = true;
                *current_seat = '#';
            } else if prev_seat == '#' && neighbor_count >= required_neighbor_count {
                changed = true;
                *current_seat = 'L';
            } else {
                *current_seat = prev_seat;
            }
        }

        match changed {
            true => None,
            false => Some(self.grid.iter().filter(|state| **state == '#').count()),
        }
    }

    #[inline]
    fn index_to_coordinates(&self, index: usize) -> Option<(usize, usize)> {
        match index.cmp(&self.size) {
            Ordering::Less => Some((index / self.stride, index % self.stride)),
            _ => None,
        }
    }

    #[inline]
    fn coordinates_to_index(&self, row: usize, col: usize) -> Option<usize> {
        Some(col)
            .filter(|col| col < &self.stride)
            .map(|col| row * self.stride + col)
            .filter(|index| index < &self.size)
    }

    #[inline]
    fn cast_rays(&self, source: usize) -> Vec<Vec<char>> {
        let ray_directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        ray_directions
            .iter()
            .map(|(delta_row, delta_col)| {
                let (mut src_row, mut src_col) = self.index_to_coordinates(source).unwrap();
                let mut ray = Vec::new();
                loop {
                    let dst_row = overloaded_checked_add(src_row, *delta_row);
                    let dst_col = overloaded_checked_add(src_col, *delta_col);
                    if let Some(seat) = dst_row
                        .zip(dst_col)
                        .and_then(|(r, c)| self.coordinates_to_index(r, c))
                        .and_then(|i| self.grid.get(i))
                    {
                        ray.push(seat);
                        src_row = dst_row.unwrap();
                        src_col = dst_col.unwrap();
                        if *seat != '.' {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                ray.into_iter().copied().collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }
}

fn main() {
    let answer1 = {
        let mut grid = Grid::new();
        loop {
            if let Some(result) = grid.play_once(
                |ray| ray.iter().next().map(|it| *it == '#').unwrap_or(false),
                4,
            ) {
                break result;
            }
        }
    };

    let answer2 = {
        let mut grid = Grid::new();
        loop {
            if let Some(result) = grid.play_once(
                |ray| {
                    ray.iter()
                        .find(|x| **x != '.')
                        .map(|it| *it == '#')
                        .unwrap_or(false)
                },
                5,
            ) {
                break result;
            }
        }
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

#[inline]
fn overloaded_checked_add(num: usize, rhs: i8) -> Option<usize> {
    match rhs.cmp(&0) {
        Ordering::Less => num.checked_sub(rhs.unsigned_abs() as usize),
        Ordering::Equal => Some(num),
        Ordering::Greater => num.checked_add(rhs.unsigned_abs() as usize),
    }
}
