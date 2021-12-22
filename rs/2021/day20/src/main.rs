use advent_of_code::read_input_as_string;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

type Point = (isize, isize);

#[derive(Clone, Debug)]
struct Rect {
    lo_row: isize,
    lo_col: isize,
    hi_row: isize,
    hi_col: isize,
}

impl Rect {
    fn contains(&self, point: &Point) -> bool {
        let (row, col) = *point;
        self.lo_row <= row && row <= self.hi_row && self.lo_col <= col && col <= self.hi_col
    }

    fn on_edge_or_outside(&self, point: &Point) -> bool {
        let (row, col) = *point;
        let on_horizontal_edge = (self.lo_col..=self.hi_col).contains(&col)
            && (row == self.lo_row || row == self.hi_row);
        let on_vertical_edge = (self.lo_row..=self.hi_row).contains(&row)
            && (col == self.lo_col || col == self.hi_col);
        on_horizontal_edge || on_vertical_edge || !self.contains(point)
    }

    fn expand(&self, increase: isize) -> Self {
        Self {
            lo_row: self.lo_row - increase,
            lo_col: self.lo_col - increase,
            hi_row: self.hi_row + increase,
            hi_col: self.hi_col + increase,
        }
    }

    fn iter(&self) -> impl Iterator<Item = (isize, isize)> {
        let lo_row = self.lo_row;
        let hi_row = self.hi_row;
        let lo_col = self.lo_col;
        let hi_col = self.hi_col;
        (lo_row..=hi_row).flat_map(move |row| (lo_col..=hi_col).map(move |col| (row, col)))
    }

    fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = (isize, isize)>> {
        let lo_row = self.lo_row;
        let hi_row = self.hi_row;
        let lo_col = self.lo_col;
        let hi_col = self.hi_col;
        (lo_row..=hi_row).map(move |row| (lo_col..=hi_col).map(move |col| (row, col)))
    }
}

#[derive(Clone, Debug)]
struct Image {
    pixels_on: HashSet<Point>,
    dimensions: Rect,
    enhance_iteration_toggle: bool,
    enhance_algorithm: Vec<char>,
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .dimensions
            .rows()
            .map(|row| {
                row.map(|point| {
                    if self.pixels_on.contains(&point) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", s)
    }
}

impl Image {
    fn create_enhanced_image(self) -> Image {
        let enhanced_rect = self.dimensions.expand(1);
        let new_pixels = enhanced_rect
            .iter()
            .filter(|point| {
                let algorithm_index = self.get_encoded_enhanced_pixel(*point, &enhanced_rect);
                *self.enhance_algorithm.get(algorithm_index).unwrap() == '#'
            })
            .collect::<HashSet<Point>>();

        Image {
            pixels_on: new_pixels,
            dimensions: enhanced_rect,
            enhance_iteration_toggle: !self.enhance_iteration_toggle,
            enhance_algorithm: self.enhance_algorithm,
        }
    }

    #[inline]
    fn get_encoded_enhanced_pixel(&self, center: Point, enhanced_rect: &Rect) -> usize {
        let edge_or_beyond_value = if self.distance_points_will_toggle() {
            if self.enhance_iteration_toggle {
                1
            } else {
                0
            }
        } else {
            0
        };

        let (row, col) = center;
        [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]
        .iter()
        .map(|point| {
            if enhanced_rect.on_edge_or_outside(point) {
                edge_or_beyond_value
            } else if self.pixels_on.contains(point) {
                1
            } else {
                0
            }
        })
        .fold(0usize, |acc, bit| (acc << 1) + bit)
    }

    #[inline]
    fn distance_points_will_toggle(&self) -> bool {
        self.enhance_algorithm
            .first()
            .zip(self.enhance_algorithm.last())
            .map(|(first, last)| *first == '#' && *last == '.')
            .unwrap_or(false)
    }
}

fn main() {
    let raw = read_input_as_string("2021/day20/src/input.txt");
    let image = {
        let (raw_algorithm, raw_image) = raw.split_once("\n\n").unwrap();
        let algorithm = raw_algorithm.chars().collect::<Vec<_>>();
        let image = raw_image
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, pixel)| *pixel == '#')
                    .map(move |(col, _)| (row as isize, col as isize))
            })
            .collect::<HashSet<Point>>();
        let dimensions = Rect {
            lo_row: 0,
            lo_col: 0,
            hi_row: *image.iter().map(|(row, _)| row).max().unwrap(),
            hi_col: *image.iter().map(|(_, col)| col).max().unwrap(),
        };

        Image {
            pixels_on: image,
            dimensions,
            enhance_iteration_toggle: false,
            enhance_algorithm: algorithm,
        }
    };

    let answer1 = image
        .clone()
        .create_enhanced_image()
        .create_enhanced_image()
        .pixels_on
        .len();

    let answer2 = (0..50)
        .fold(image, |acc, _| acc.create_enhanced_image())
        .pixels_on
        .len();

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
