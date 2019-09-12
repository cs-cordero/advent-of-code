use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let steps = get_steps("src/input.txt")?;
    let mut current_direction = Direction::North;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut first_return: Option<(i32, i32)> = None;

    let mut x = 0;
    let mut y = 0;
    for step in steps.iter() {
        let direction = match step.get(0..1) {
            Some(value) => value,
            None => panic!("Bad Input."),
        };
        match direction {
            "L" => current_direction = current_direction.turn_left(),
            "R" => current_direction = current_direction.turn_right(),
            _ => panic!("Invariant."),
        }

        let movement = step.get(1..).unwrap().parse::<i32>().unwrap();
        for _i in 0..movement {
            match current_direction {
                Direction::North => x += 1,
                Direction::South => x -= 1,
                Direction::East => y += 1,
                Direction::West => y -= 1,
            };
            let current_position = (x, y);
            if first_return == None && visited.contains(&current_position) {
                first_return = Some(current_position);
            };
            visited.insert(current_position);
        }
    }
    println!("Position: {} ({}, {})", current_direction, x, y);
    println!("Part 1 Answer: {}", x.abs() + y.abs());
    println!(
        "Part 2 Answer: {:?}",
        first_return.unwrap().0.abs() + first_return.unwrap().1.abs()
    );
    return Ok(());
}

pub enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        return match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
    }
    pub fn turn_right(&self) -> Direction {
        return match self {
            Direction::North => Direction::East,
            Direction::West => Direction::North,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
        };
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
            Direction::West => write!(f, "West"),
            Direction::East => write!(f, "East"),
        }
    }
}

fn get_steps(path: &str) -> io::Result<Vec<String>> {
    let contents = read_input(path)?;
    let steps = contents
        .trim_end()
        .split(", ")
        .map(|s| s.to_string())
        .collect();
    return Ok(steps);
}

fn read_input(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    return Ok(contents);
}
