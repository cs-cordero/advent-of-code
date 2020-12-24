use advent_of_code::*;
use std::collections::HashSet;

static HEX_NEIGHBOR_DELTAS: [(i32, i32); 6] = [(-1, 1), (0, 1), (-1, 0), (1, 0), (0, -1), (1, -1)];

fn main() {
    let input = read_input_as_lines("2020/day24/src/input.txt")
        .iter()
        .map(|s| parse_line(s))
        .map(find_coordinate)
        .collect::<Vec<_>>();

    let mut black_tiles: HashSet<(i32, i32)> = HashSet::new();
    for coordinate in input.iter() {
        if black_tiles.contains(coordinate) {
            black_tiles.remove(coordinate);
        } else {
            black_tiles.insert(*coordinate);
        }
    }

    let answer1 = black_tiles.len();

    let answer2 = {
        for _ in 0..100 {
            let mut next_black_tiles: HashSet<(i32, i32)> = HashSet::new();
            let mut white_tiles: HashSet<(i32, i32)> = HashSet::new();

            for (black_tile_x, black_tile_y) in black_tiles.iter() {
                let mut black_neighbors = 0;
                for (delta_x, delta_y) in HEX_NEIGHBOR_DELTAS.iter() {
                    let coordinate = (black_tile_x + delta_x, black_tile_y + delta_y);
                    if black_tiles.contains(&coordinate) {
                        black_neighbors += 1;
                    } else {
                        white_tiles.insert(coordinate);
                    }
                }
                if black_neighbors == 1 || black_neighbors == 2 {
                    next_black_tiles.insert((*black_tile_x, *black_tile_y));
                }
            }

            for (white_tile_x, white_tile_y) in white_tiles {
                let black_neighbor_count = HEX_NEIGHBOR_DELTAS
                    .iter()
                    .filter(|(delta_x, delta_y)| {
                        black_tiles.contains(&(white_tile_x + *delta_x, white_tile_y + *delta_y))
                    })
                    .count();

                if black_neighbor_count == 2 {
                    next_black_tiles.insert((white_tile_x, white_tile_y));
                }
            }

            black_tiles = next_black_tiles;
        }
        black_tiles.len()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn parse_line(s: &str) -> Vec<String> {
    let mut iter = s.chars();
    let mut result = Vec::new();
    while let Some(direction) = iter.next() {
        match direction {
            'e' | 'w' => result.push(direction.to_string()),
            _ => {
                let direction2 = iter.next().unwrap();
                result.push([direction.to_string(), direction2.to_string()].join(""));
            }
        }
    }
    result
}

fn find_coordinate(path: Vec<String>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;
    for step in path.iter() {
        match step.as_str() {
            "ne" => {
                x += 0;
                y += 1;
            }
            "e" => {
                x += 1;
                y += 0;
            }
            "se" => {
                x += 1;
                y += -1;
            }
            "sw" => {
                x += 0;
                y += -1;
            }
            "w" => {
                x += -1;
                y += 0;
            }
            "nw" => {
                x += -1;
                y += 1;
            }
            _ => panic!("Invalid"),
        }
    }
    (x, y)
}
