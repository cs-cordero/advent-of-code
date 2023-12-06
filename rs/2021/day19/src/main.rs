use advent_of_code::*;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    const fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 { x, y, z }
    }

    fn face_pos_z(&self) -> Vector3 {
        *self
    }

    fn face_neg_z(&self) -> Vector3 {
        Vector3::new(-self.x, self.y, -self.z)
    }

    fn face_neg_x(&self) -> Vector3 {
        Vector3::new(self.z, self.y, -self.x)
    }

    fn face_pos_x(&self) -> Vector3 {
        Vector3::new(-self.z, self.y, self.x)
    }

    fn face_neg_y(&self) -> Vector3 {
        Vector3::new(self.x, self.z, -self.y)
    }

    fn face_pos_y(&self) -> Vector3 {
        Vector3::new(self.x, -self.z, self.y)
    }

    fn rotate_90_z(&self) -> Vector3 {
        Vector3::new(-self.y, self.x, self.z)
    }
}

/// 3, 4, 5 when facing -z means:
///     in the z direction, z = -3
///     in the +y direction, y = 4
///     in the +x direction, x = 5

macro_rules! add_vector3 {
    ($(($lhs: ty, $rhs: ty))*) => {
        $(
            impl Add<$rhs> for $lhs {
                type Output = Vector3;

                 fn add(self, rhs: $rhs) -> Self::Output {
                    Vector3::new(
                        self.x + rhs.x,
                        self.y + rhs.y,
                        self.z + rhs.z,
                    )
                }
            }
        )*
    }
}

macro_rules! add_assign_vector3 {
    ($(($lhs: ty, $rhs: ty))*) => {
        $(
            impl AddAssign<$rhs> for $lhs {
                fn add_assign(&mut self, rhs: $rhs) {
                    self.x += rhs.x;
                    self.y += rhs.y;
                    self.z += rhs.z;
                }
            }
        )*
    }
}

add_vector3! {
    (Vector3, Vector3)
    (Vector3, &'_ Vector3)
    (&'_ Vector3, Vector3)
    (&'_ Vector3, &'_ Vector3)
}

add_assign_vector3! {
    (Vector3, Vector3)
    (Vector3, &'_ Vector3)
}

macro_rules! sub_vector3 {
    ($(($lhs: ty, $rhs: ty))*) => {
        $(
            impl Sub<$rhs> for $lhs {
                type Output = Vector3;

                fn sub(self, rhs: $rhs) -> Self::Output {
                    Vector3::new(
                        self.x - rhs.x,
                        self.y - rhs.y,
                        self.z - rhs.z,
                    )
                }
            }
        )*
    }
}

macro_rules! sub_assign_vector3 {
    ($(($lhs: ty, $rhs: ty))*) => {
        $(
            impl SubAssign<$rhs> for $lhs {
                fn sub_assign(&mut self, rhs: $rhs) {
                    self.x -= rhs.x;
                    self.y -= rhs.y;
                    self.z -= rhs.z;
                }
            }
        )*
    }
}

sub_vector3! {
    (Vector3, Vector3)
    (Vector3, &'_ Vector3)
    (&'_ Vector3, Vector3)
    (&'_ Vector3, &'_ Vector3)
}

sub_assign_vector3! {
    (Vector3, Vector3)
    (Vector3, &'_ Vector3)
}

struct Scanner {
    id: usize,
    beacons: HashSet<Vector3>,
    rotation_configuration: u8,
}

impl Scanner {
    const ROTATION_CONFIGURATION_COUNT: u8 = 24;

    fn from_lines<'a>(lines: impl IntoIterator<Item = &'a str>) -> Self {
        let mut lines_iter = lines.into_iter();
        let id = {
            let (_, num_raw) = (lines_iter.next().unwrap() as &str)
                .split_once("scanner ")
                .unwrap();
            let (num_raw, _) = num_raw.split_once(' ').unwrap();
            num_raw.parse::<usize>().unwrap()
        };
        let beacons = lines_iter
            .map(|line| {
                let mut coordinates = line.split(',').map(|elem| elem.parse::<i32>().unwrap());
                Vector3::new(
                    coordinates.next().unwrap(),
                    coordinates.next().unwrap(),
                    coordinates.next().unwrap(),
                )
            })
            .collect::<HashSet<_>>();
        Scanner {
            id,
            beacons,
            rotation_configuration: 0,
        }
    }

    fn combine(&mut self, rhs: &Self) {
        self.beacons = self.beacons.union(&rhs.beacons).copied().collect();
    }

    fn brute_force_has_overlap(&self, rhs: &mut Self) -> Option<Vector3> {
        if let Some(location) = self.overlaps_with(rhs) {
            return Some(location);
        }

        for _ in 0..Scanner::ROTATION_CONFIGURATION_COUNT {
            rhs.rotate();

            if let Some(location) = self.overlaps_with(rhs) {
                return Some(location);
            }
        }

        self.overlaps_with(rhs)
    }

    /// Without rotating the scanner, translate each point to
    /// overlap with one other point, then confirm whether the intersection
    /// has at least 12 points overlapping.
    fn overlaps_with(&self, rhs: &mut Self) -> Option<Vector3> {
        let rhs_beacons = rhs.beacons.iter().copied().collect::<Vec<_>>();
        for lhs_beacon in self.beacons.iter() {
            for rhs_beacon in rhs_beacons.iter() {
                let translation = lhs_beacon - rhs_beacon;
                let right = rhs
                    .beacons
                    .iter()
                    .map(|beacon| beacon + translation)
                    .collect::<HashSet<_>>();
                let overlap_size = self.beacons.intersection(&right).count();
                if overlap_size >= 12 {
                    rhs.translate(translation);
                    return Some(translation);
                }
            }
        }

        None
    }

    fn translate(&mut self, translation: Vector3) {
        self.beacons = self
            .beacons
            .iter()
            .map(|beacon| beacon + translation)
            .collect();
    }

    fn rotate(&mut self) {
        match self.rotation_configuration {
            0 => {
                self.beacons = self.beacons.iter().map(|b| b.face_pos_z()).collect();
            }
            1 => self.rotate_all_vectors_z(),
            2 => self.rotate_all_vectors_z(),
            3 => self.rotate_all_vectors_z(),
            4 => {
                self.rotate_all_vectors_z();
                self.beacons = self.beacons.iter().map(|b| b.face_neg_z()).collect();
            }
            5 => self.rotate_all_vectors_z(),
            6 => self.rotate_all_vectors_z(),
            7 => self.rotate_all_vectors_z(),
            8 => {
                self.rotate_all_vectors_z();
                self.beacons = self.beacons.iter().map(|b| b.face_neg_x()).collect();
            }
            9 => self.rotate_all_vectors_z(),
            10 => self.rotate_all_vectors_z(),
            11 => self.rotate_all_vectors_z(),
            12 => {
                self.rotate_all_vectors_z();
                self.beacons = self.beacons.iter().map(|b| b.face_pos_x()).collect();
            }
            13 => self.rotate_all_vectors_z(),
            14 => self.rotate_all_vectors_z(),
            15 => self.rotate_all_vectors_z(),
            16 => {
                self.rotate_all_vectors_z();
                self.beacons = self.beacons.iter().map(|b| b.face_neg_y()).collect();
            }
            17 => self.rotate_all_vectors_z(),
            18 => self.rotate_all_vectors_z(),
            19 => self.rotate_all_vectors_z(),
            20 => {
                self.rotate_all_vectors_z();
                self.beacons = self.beacons.iter().map(|b| b.face_pos_y()).collect();
            }
            21 => self.rotate_all_vectors_z(),
            22 => self.rotate_all_vectors_z(),
            23 => self.rotate_all_vectors_z(),
            _ => unreachable!("There are only 24 rotations."),
        }
        self.rotation_configuration += 1;
        self.rotation_configuration %= Scanner::ROTATION_CONFIGURATION_COUNT;
    }

    #[inline]
    fn rotate_all_vectors_z(&mut self) {
        self.beacons = self
            .beacons
            .iter()
            .map(|beacon| beacon.rotate_90_z())
            .collect();
    }
}

fn main() {
    let raw = read_input_as_string("2021/day19/src/input.txt");
    let mut scanners = raw
        .trim()
        .split("\n\n")
        .map(|chunk| Scanner::from_lines(chunk.split('\n')))
        .collect::<Vec<_>>();

    let mut scanner_locations = Vec::new();

    let answer1 = {
        let (base, remaining) = scanners.split_at_mut(1);
        let base = base.get_mut(0).unwrap();
        let mut remaining = remaining.iter_mut().collect::<Vec<_>>();

        while !remaining.is_empty() {
            for i in 0..remaining.len() {
                let scanner = remaining.get_mut(i).unwrap();
                if let Some(location) = base.brute_force_has_overlap(scanner) {
                    scanner_locations.push(location);
                    let scanner = remaining.swap_remove(i);
                    base.combine(scanner);
                    println!(
                        "Found overlap with scanner {}.  {} scanners remaining.",
                        scanner.id,
                        remaining.len()
                    );
                    break;
                } else {
                    println!("No overlap found with scanner {}", scanner.id);
                }
            }
        }

        base.beacons.len()
    };

    let answer2 = {
        let mut furthest_distance = 0;
        for l in 0..scanner_locations.len() - 1 {
            for r in l + 1..scanner_locations.len() {
                let lhs = scanner_locations.get(l).unwrap();
                let rhs = scanner_locations.get(r).unwrap();
                let distance =
                    (lhs.x - rhs.x).abs() + (lhs.y - rhs.y).abs() + (lhs.z - rhs.z).abs();
                furthest_distance = max(furthest_distance, distance);
            }
        }
        furthest_distance
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
