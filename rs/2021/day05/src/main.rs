use advent_of_code::*;
use std::cmp::Ordering;
use std::collections::HashMap;

type Point = (i32, i32);

fn main() {
    let data = read_input_as_lines("2021/day05/src/input.txt")
        .into_iter()
        .map(|line| {
            let (begin, end) = line.split_once(" -> ").unwrap();
            let (x1, y1) = begin.split_once(",").unwrap();
            let (x2, y2) = end.split_once(",").unwrap();
            let x1 = x1.parse::<i32>().unwrap();
            let y1 = y1.parse::<i32>().unwrap();
            let x2 = x2.parse::<i32>().unwrap();
            let y2 = y2.parse::<i32>().unwrap();
            ((x1, y1), (x2, y2))
        })
        .collect::<Vec<(Point, Point)>>();

    let answer1 = {
        let horizontal_and_vertical_only = data
            .iter()
            .copied()
            .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
            .collect::<Vec<_>>();

        get_overlap_count(&horizontal_and_vertical_only)
    };

    let answer2 = get_overlap_count(&data);

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn get_direction(begin: i32, end: i32) -> i32 {
    match begin.cmp(&end) {
        Ordering::Greater => -1,
        Ordering::Equal => 0,
        Ordering::Less => 1,
    }
}

fn draw_line(point_to_frequency: &mut HashMap<Point, i32>, begin: Point, end: Point) {
    let (mut x, mut y) = begin;
    let direction_x = get_direction(x, end.0);
    let direction_y = get_direction(y, end.1);

    loop {
        *point_to_frequency.entry((x, y)).or_insert(0) += 1;

        if (x, y) == end {
            break;
        }

        x += direction_x;
        y += direction_y;
    }
}

fn get_overlap_count(data: &[(Point, Point)]) -> usize {
    let mut freq = HashMap::new();
    for (begin, end) in data {
        draw_line(&mut freq, *begin, *end);
    }

    freq.values()
        .into_iter()
        .filter(|count| **count > 1)
        .count()
}
