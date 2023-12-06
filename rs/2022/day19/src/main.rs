extern crate core;

use advent_of_code::*;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    bot_cost_ore: HashMap<Resource, u32>,
    bot_cost_clay: HashMap<Resource, u32>,
    bot_cost_obsidian: HashMap<Resource, u32>,
    bot_cost_geode: HashMap<Resource, u32>,
}

impl Blueprint {
    fn get_cost_for(&self, resource: &Resource) -> &HashMap<Resource, u32> {
        match resource {
            Resource::Ore => &self.bot_cost_ore,
            Resource::Clay => &self.bot_cost_clay,
            Resource::Obsidian => &self.bot_cost_obsidian,
            Resource::Geode => &self.bot_cost_geode,
        }
    }
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (blueprint, rest) = s.split_once(": ").unwrap();
        let (_, id) = blueprint.split_once(' ').unwrap();
        let id = id.parse::<usize>().unwrap();

        let mut costs = rest.split(". ");
        let raw_ore = costs.next().unwrap();
        let raw_clay = costs.next().unwrap();
        let raw_obsidian = costs.next().unwrap();
        let raw_geode = costs.next().unwrap();

        Ok(Blueprint {
            id,
            bot_cost_ore: process_raw_str(raw_ore),
            bot_cost_clay: process_raw_str(raw_clay),
            bot_cost_obsidian: process_raw_str(raw_obsidian),
            bot_cost_geode: process_raw_str(raw_geode),
        })
    }
}

fn process_raw_str(s: &str) -> HashMap<Resource, u32> {
    let mut map = HashMap::new();

    let (_, costs) = s.split_once(" costs ").unwrap();
    for resource in costs.split(" and ") {
        let (count, resource_type) = resource.split_once(' ').unwrap();
        let count: u32 = count.parse().unwrap();

        if resource_type.starts_with("ore") {
            map.insert(Resource::Ore, count);
        } else if resource_type.starts_with("clay") {
            map.insert(Resource::Clay, count);
        } else if resource_type.starts_with("obsidian") {
            map.insert(Resource::Obsidian, count);
        } else if resource_type.starts_with("geode") {
            map.insert(Resource::Geode, count);
        } else {
            panic!("Unknown resource type {:?}", resource_type);
        }
    }

    map
}

fn play(blueprint: &Blueprint, max_minutes: u32) -> u32 {
    let mut best = 0;
    play_helper(
        &mut best,
        blueprint,
        max_minutes,
        (1, 0, 0, 0),
        (0, 0, 0, 0),
        0,
    )
}

fn play_helper(
    best: &mut u32,
    blueprint: &Blueprint,
    max_minutes: u32,
    robot_counts: (u32, u32, u32, u32),
    resource_counts: (u32, u32, u32, u32),
    current_minute: u32,
) -> u32 {
    let (max_ore_bots, max_clay_bots, max_obsidian_bots, max_geode_bots) =
        get_max_bot_counts(blueprint);

    match current_minute.cmp(&max_minutes) {
        Ordering::Equal => {
            let geode_count = resource_counts.3;
            if geode_count > *best {
                *best = geode_count;
            }
            return *best;
        }
        Ordering::Greater => panic!(),
        Ordering::Less => {}
    }

    if !rudimentary_filter(
        best,
        current_minute,
        max_minutes,
        robot_counts.3,
        resource_counts.3,
    ) {
        return *best;
    }

    let (robot_ore, robot_clay, robot_obsidian, robot_geode) = robot_counts;
    let (ore, clay, obsidian, geode) = resource_counts;

    let mut max_geodes = 0;
    for desired_bot_type in [
        Resource::Geode,
        Resource::Obsidian,
        Resource::Clay,
        Resource::Ore,
    ] {
        // if we've hit the max bots needed for this desired bot type.
        if match desired_bot_type {
            Resource::Ore => robot_ore >= max_ore_bots,
            Resource::Clay => robot_clay >= max_clay_bots,
            Resource::Obsidian => robot_obsidian >= max_obsidian_bots,
            Resource::Geode => robot_geode >= max_geode_bots,
        } {
            continue;
        }

        let cost = blueprint.get_cost_for(&desired_bot_type);

        if let Some(minutes) = minutes_to_generate_robot(cost, resource_counts, robot_counts) {
            let minutes = min(max_minutes - current_minute, minutes);

            let mut next_minutes = current_minute + minutes;

            let mut next_ore = ore + robot_ore * minutes;
            let mut next_clay = clay + robot_clay * minutes;
            let mut next_obsidian = obsidian + robot_obsidian * minutes;
            let mut next_geode = geode + robot_geode * minutes;

            let mut next_robot_ore = robot_ore;
            let mut next_robot_clay = robot_clay;
            let mut next_robot_obsidian = robot_obsidian;
            let mut next_robot_geode = robot_geode;

            if current_minute + minutes < max_minutes {
                next_minutes += 1;

                next_ore += robot_ore;
                next_clay += robot_clay;
                next_obsidian += robot_obsidian;
                next_geode += robot_geode;

                match desired_bot_type {
                    Resource::Ore => {
                        next_robot_ore += 1;
                    }
                    Resource::Clay => {
                        next_robot_clay += 1;
                    }
                    Resource::Obsidian => {
                        next_robot_obsidian += 1;
                    }
                    Resource::Geode => {
                        next_robot_geode += 1;
                    }
                };
                for (spent_resource, spent_quantity) in cost.iter() {
                    match spent_resource {
                        Resource::Ore => {
                            next_ore -= spent_quantity;
                        }
                        Resource::Clay => {
                            next_clay -= spent_quantity;
                        }
                        Resource::Obsidian => {
                            next_obsidian -= spent_quantity;
                        }
                        Resource::Geode => {
                            next_geode -= spent_quantity;
                        }
                    }
                }
            }

            let next_robot_counts = (
                next_robot_ore,
                next_robot_clay,
                next_robot_obsidian,
                next_robot_geode,
            );
            let next_counts = (next_ore, next_clay, next_obsidian, next_geode);

            max_geodes = max(
                max_geodes,
                play_helper(
                    best,
                    blueprint,
                    max_minutes,
                    next_robot_counts,
                    next_counts,
                    next_minutes,
                ),
            );
        }
    }
    max_geodes
}

fn rudimentary_filter(
    best: &u32,
    current_minutes: u32,
    max_minutes: u32,
    geode_robot_count: u32,
    geode_count: u32,
) -> bool {
    let mut count = geode_count;
    let mut robots = geode_robot_count;
    for _ in current_minutes..=max_minutes {
        count += robots;
        robots += 1;
    }

    count > *best
}

fn minutes_to_generate_robot(
    cost: &HashMap<Resource, u32>,
    resources: (u32, u32, u32, u32),
    robots: (u32, u32, u32, u32),
) -> Option<u32> {
    let mut result = -1;

    for (resource, &required_amount) in cost.iter() {
        let (current_in_hand, current_delta) = match resource {
            Resource::Ore => (resources.0, robots.0),
            Resource::Clay => (resources.1, robots.1),
            Resource::Obsidian => (resources.2, robots.2),
            Resource::Geode => (resources.3, robots.3),
        };

        if current_in_hand < required_amount && current_delta == 0 {
            return None;
        }

        let minutes = if current_in_hand >= required_amount {
            0
        } else {
            f32::ceil((required_amount - current_in_hand) as f32 / current_delta as f32) as i32
        };

        result = max(result, minutes);
    }

    if result >= 0 {
        Some(result as u32)
    } else {
        panic!();
    }
}

fn get_max_bot_counts(blueprint: &Blueprint) -> (u32, u32, u32, u32) {
    let mut ore = 0;
    let mut clay = 0;
    let mut obsidian = 0;

    blueprint
        .bot_cost_ore
        .iter()
        .chain(&blueprint.bot_cost_clay)
        .chain(&blueprint.bot_cost_obsidian)
        .chain(&blueprint.bot_cost_geode)
        .for_each(|(resource, &count)| match resource {
            Resource::Ore => ore = max(ore, count),
            Resource::Clay => clay = max(clay, count),
            Resource::Obsidian => obsidian = max(obsidian, count),
            _ => {}
        });

    (ore, clay, obsidian, u32::MAX)
}

fn main() {
    let input = read_input_as_lines("2022/day19/src/input.txt")
        .into_iter()
        .map(|line| Blueprint::from_str(&line).unwrap())
        .collect::<Vec<_>>();

    let solution1 = input
        .iter()
        .map(|blueprint| {
            let id = blueprint.id;
            let geode_count = play(blueprint, 24);
            id as u32 * geode_count
        })
        .sum::<u32>();

    let solution2 = input
        .iter()
        .take(3)
        .map(|blueprint| play(blueprint, 32))
        .product::<u32>();

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}
