extern crate core;

use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use advent_of_code::*;

#[derive(Copy, Clone)]
struct Sensor {
    x: isize,
    y: isize,
    min_dist_to_beacon: usize
}

impl Sensor {
    fn within_min_range(&self, point: &Point) -> bool {
        ((point.x - self.x).abs() + (point.y - self.y).abs()) as usize <= self.min_dist_to_beacon
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: isize,
    y: isize
}

fn main() {
    let (sensors, beacons) = {
        let mut sensors: Vec<Sensor> = Vec::new();
        let mut beacons: HashSet<Point> = HashSet::new();

        for line in read_input_as_lines("2022/day15/src/input.txt") {
            let (sensor_data, beacon_data) = line.split_once(": closest beacon is at ").unwrap();

            let beacon = {
                let (beacon_data_x, beacon_data_y) = beacon_data.split_once(", ").unwrap();
                let (_, beacon_data_x) = beacon_data_x.split_once('=').unwrap();
                let (_, beacon_data_y) = beacon_data_y.split_once('=').unwrap();
                Point {
                    x: beacon_data_x.parse().unwrap(),
                    y: beacon_data_y.parse().unwrap()
                }
            };

            let sensor = {
                let (_, sensor_data) = sensor_data.split_once(" at ").unwrap();
                let (sensor_data_x, sensor_data_y) = sensor_data.split_once(", ").unwrap();
                let (_, sensor_data_x) = sensor_data_x.split_once('=').unwrap();
                let (_, sensor_data_y) = sensor_data_y.split_once('=').unwrap();
                let x: isize = sensor_data_x.parse().unwrap();
                let y: isize = sensor_data_y.parse().unwrap();

                let dist = ((beacon.x - x).abs() + (beacon.y - y).abs()) as usize;

                Sensor { x, y, min_dist_to_beacon: dist }
            };

            sensors.push(sensor);
            beacons.insert(beacon);
        }

        (sensors, beacons)
    };

    let solution1 = {
        let target_y = 2000000;

        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        for sensor in sensors.iter() {
            let delta_x = sensor.min_dist_to_beacon as isize - (sensor.y - target_y).abs();

            if delta_x < 0 {
                continue;
            }

            min_x = min(min_x, sensor.x - delta_x);
            max_x = max(max_x, sensor.x + delta_x);
        }

        let beacons_on_row = beacons.iter().filter(|beacon| beacon.y == target_y).count();

        max_x - min_x + 1 - beacons_on_row as isize
    };

    let solution2 = {
        let target = quad_find(&sensors).unwrap();
        target.x * 4000000 + target.y
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn quad_find(sensors: &[Sensor]) -> Option<Point> {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 4000000, 4000000));

    while let Some(quad) = queue.pop_front() {
        let (min_x, min_y, max_x, max_y) = quad;

        let any_sensors_reach_quad = sensors
            .iter()
            .any(|sensor| {
                if min_x <= sensor.x && sensor.x <= max_x && min_y <= sensor.y && sensor.y <= max_y {
                    // sensor is inside region
                    return true;
                }

                // find closest point in rectangular region to "circular" sensor range
                let nearest_x = max(min_x, min(sensor.x, max_x));
                let nearest_y = max(min_y, min(sensor.y, max_y));
                let nearest = Point { x: nearest_x, y: nearest_y };

                // check if the closest point is within range
                sensor.within_min_range(&nearest)
            });

        if !any_sensors_reach_quad {
            assert_eq!(min_x, max_x);
            assert_eq!(min_y, max_y);

            return Some(Point { x: min_x, y: min_y })
        }

        if fully_covered_by_any_sensor(quad, sensors) {
            continue;
        }

        queue.extend(make_quads(min_x, min_y, max_x, max_y));
    }

    None
}

fn make_quads(min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> [(isize, isize, isize, isize); 4] {
    let half_x_dist = (max_x - min_x) / 2;
    let half_y_dist = (max_y - min_y) / 2;

    let top_left_quad = (min_x, min_y, min_x + half_x_dist, min_y + half_y_dist);
    let top_right_quad = (min_x + half_x_dist + 1, min_y, max_x, min_y + half_y_dist);
    let bottom_left_quad = (min_x, min_y + half_y_dist + 1, min_x + half_x_dist, max_y);
    let bottom_right_quad = (min_x + half_x_dist + 1, min_y + half_y_dist + 1, max_x, max_y);

    [top_left_quad, top_right_quad, bottom_left_quad, bottom_right_quad]
}

fn fully_covered_by_any_sensor(quad: (isize, isize, isize, isize), sensors: &[Sensor]) -> bool {
    let (ax, ay, bx, by) = quad;

    let corners = [
        Point { x: ax, y: ay },
        Point { x: bx, y: ay },
        Point { x: ax, y: by },
        Point { x: bx, y: by }
    ];

    sensors.iter().any(|sensor| corners.iter().all(|point| sensor.within_min_range(point)))
}
