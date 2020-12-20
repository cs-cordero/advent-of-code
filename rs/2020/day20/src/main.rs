use advent_of_code::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

type Grid<T> = Vec<Vec<T>>;
type Image = Vec<Vec<char>>;

fn main() {
    let data: HashMap<u32, Image> = read_input_as_string("2020/day20/src/input.txt")
        .split("\n\n")
        .map(|tile| {
            let mut tile_iter = tile.lines();
            let (_, id) = split_once_from_right(tile_iter.next().unwrap(), " ");
            let tile_id = id[..id.len() - 1].parse::<u32>().unwrap();
            let tile = tile_iter
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();
            (tile_id, tile)
        })
        .collect();

    let size = (data.len() as f32).sqrt() as usize;
    let mut grid: Grid<(u32, Image)> = vec![vec![(0, vec![]); size]; size];
    solve_jigsaw(&mut grid, &data, HashSet::new(), 0, 0);

    let answer1 = {
        grid[0][0].0 as u64
            * grid[0][size - 1].0 as u64
            * grid[size - 1][0].0 as u64
            * grid[size - 1][size - 1].0 as u64
    };

    let answer2 = {
        let mut count = 0;
        if let Some(image) = find_orientation_matching_condition(&build_image(&grid), |img| {
            count = count_monsters(img);
            count > 0
        }) {
            let hash_count = image.iter().flatten().filter(|&&c| c == '#').count();
            hash_count - (count * 15)
        } else {
            panic!("Failed to find any monsters!");
        }
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn get_edges(image: &[Vec<char>]) -> (u32, u32, u32, u32) {
    (
        // top
        get_bits(image[0].iter().copied()),
        // bottom
        get_bits(image[image.len() - 1].iter().copied()),
        // left
        get_bits(image.iter().map(|row| row[0])),
        // right
        get_bits(image.iter().map(|row| row[row.len() - 1])),
    )
}

fn solve_jigsaw(
    jigsaw: &mut Grid<(u32, Image)>,
    jigsaw_pieces: &HashMap<u32, Image>,
    used_pieces: HashSet<u32>,
    row: usize,
    col: usize,
) -> bool {
    let jigsaw_size = jigsaw.len();

    if row >= jigsaw_size {
        return true;
    }

    let tile_above = row
        .checked_sub(1)
        .and_then(|r| jigsaw.get(r))
        .and_then(|r| r.get(col))
        .map(|(_, tile)| {
            let (_, tile_above, _, _) = get_edges(tile);
            tile_above
        });
    let tile_to_left = col
        .checked_sub(1)
        .and_then(|c| jigsaw.get(row).and_then(|r| r.get(c)))
        .map(|(_, tile)| {
            let (_, _, _, tile_to_left) = get_edges(tile);
            tile_to_left
        });

    let next_row = if col + 1 >= jigsaw_size { row + 1 } else { row };
    let next_col = if col + 1 >= jigsaw_size { 0 } else { col + 1 };

    for (tile_id, tile) in jigsaw_pieces.iter() {
        if used_pieces.contains(tile_id) {
            continue;
        }

        if let Some(placed_tile) = find_orientation_matching_condition(tile, |t| {
            let (top, _, left, _) = get_edges(t);
            (tile_above.is_none() || top == tile_above.unwrap())
                && (tile_to_left.is_none() || left == tile_to_left.unwrap())
        }) {
            let mut used_copy = used_pieces.clone();
            used_copy.insert(*tile_id);

            jigsaw[row][col] = (*tile_id, placed_tile);

            if solve_jigsaw(jigsaw, jigsaw_pieces, used_copy, next_row, next_col) {
                return true;
            }
        }
    }
    false
}

fn count_monsters(image: &[Vec<char>]) -> usize {
    let row_size = image.len();
    let col_size = image.iter().next().unwrap().len();

    (0..row_size)
        .flat_map(|r| (0..col_size).map(move |c| (r, c)))
        .filter(|(r, c)| find_monster(image, *r, *c))
        .count()
}

fn build_image(jigsaw: &[Vec<(u32, Image)>]) -> Image {
    jigsaw
        .iter()
        .map(|row| {
            row.iter()
                .map(|(_, image)| {
                    image[1..image.len() - 1]
                        .iter()
                        .map(|row| row[1..row.len() - 1].iter().copied().collect::<Vec<_>>())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .flat_map(combine_image_row)
        .collect::<Vec<_>>()
}

fn combine_image_row(row: Vec<Image>) -> Image {
    let image_count = row.len();
    let inner_row_count = row.get(0).unwrap().len();
    let inner_col_count = row.get(0).unwrap().get(0).unwrap().len();

    (0..inner_row_count)
        .map(|i| {
            let mut combined_row = Vec::with_capacity(image_count * inner_col_count);
            row.iter()
                .map(|image| &image[i])
                .for_each(|row_from_each_image| {
                    combined_row.extend(row_from_each_image);
                });
            combined_row
        })
        .collect::<Vec<Vec<char>>>()
}

static MONSTER_OFFSETS: [(i8, i8); 15] = [
    (0, 0),
    (1, 1),
    (1, 4),
    (0, 5),
    (0, 6),
    (1, 7),
    (1, 10),
    (0, 11),
    (0, 12),
    (1, 13),
    (1, 16),
    (0, 17),
    (0, 18),
    (0, 19),
    (-1, 18),
];

fn find_monster(image: &[Vec<char>], row: usize, col: usize) -> bool {
    MONSTER_OFFSETS
        .iter()
        .copied()
        .all(|(offset_row, offset_col)| {
            let operator = match offset_row.cmp(&0) {
                Ordering::Less => usize::checked_sub,
                _ => usize::checked_add,
            };

            operator(row, offset_row.abs() as usize)
                .zip(col.checked_add(offset_col.abs() as usize))
                .and_then(|(r, c)| image.get(r).and_then(|r| r.get(c)))
                .filter(|&&character| character == '#')
                .is_some()
        })
}

fn find_orientation_matching_condition<F: FnMut(&Image) -> bool>(
    image: &[Vec<char>],
    mut func: F,
) -> Option<Image> {
    let mut image = image.to_owned();
    for _ in 0..4 {
        if func(&image) {
            return Some(image);
        }
        image = rotate_image(image);
    }
    image = flip_image(image);
    for _ in 0..4 {
        if func(&image) {
            return Some(image);
        }
        image = rotate_image(image);
    }
    None
}

#[inline]
fn transpose_image(image: Image) -> Image {
    let row_size = image.len();
    let col_size = image.get(0).unwrap().len();
    (0..col_size)
        .map(|c| (0..row_size).map(|r| image[r][c]).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

#[inline]
fn rotate_image(image: Image) -> Image {
    let mut result = transpose_image(image);
    for row in result.iter_mut() {
        row.reverse();
    }
    result
}

#[inline]
fn flip_image(mut image: Vec<Vec<char>>) -> Vec<Vec<char>> {
    image.reverse();
    image
}

#[inline]
fn get_bits(edge: impl Iterator<Item = char>) -> u32 {
    let mut result = 0;
    for element in edge {
        result <<= 1;
        match element {
            '.' => result += 0,
            '#' => result += 1,
            _ => panic!("Invalid {}", element),
        }
    }
    result
}
