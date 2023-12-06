extern crate core;

use advent_of_code::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve {
    flow_rate: u64,
    tunnels: Vec<String>,
}

fn main() {
    let valves = {
        let mut valve_map = HashMap::new();

        for line in read_input_as_lines("2022/day16/src/input.txt") {
            let (a, b) = line.split_once("; ").unwrap();
            let (a, flow_rate) = a.split_once(" has flow rate=").unwrap();
            let (_, valve_name) = a.split_once(' ').unwrap();

            let (_, b) = b.split_once(" to ").unwrap();
            let (_, tunnels) = b.split_once(' ').unwrap();

            let valve = Valve {
                flow_rate: flow_rate.parse().unwrap(),
                tunnels: tunnels.split(", ").map(|s| s.to_string()).collect(),
            };

            valve_map.insert(valve_name.to_string(), valve);
        }

        valve_map
    };

    let solution1 = part1(&valves).unwrap();
    let solution2 = part2(&valves).unwrap();

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn part1(valve_lookup: &HashMap<String, Valve>) -> Option<u64> {
    let mut cache = HashMap::new();
    let open_valves = HashSet::new();

    dfs_helper(1, "AA", 0, 0, &open_valves, valve_lookup, &mut cache)
}

fn part2(valve_lookup: &HashMap<String, Valve>) -> Option<u64> {
    let mut cache = HashMap::new();
    let open_valves = HashSet::new();

    dfs_helper_2(1, "AA", "AA", 0, 0, &open_valves, valve_lookup, &mut cache)
}

fn dfs_helper(
    minute: u32,
    current_location: &str,
    flow_rate: u64,
    current_score: u64,
    open_valves: &HashSet<String>,
    valve_lookup: &HashMap<String, Valve>,
    cache: &mut HashMap<(u32, String, u64), u64>,
) -> Option<u64> {
    if minute > 30 {
        return Some(current_score);
    }

    let cache_key = (minute, current_location.to_string(), flow_rate);
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= current_score {
            return None;
        }
    }
    cache.insert(cache_key, current_score);

    let current_valve = valve_lookup.get(current_location).unwrap();

    let best_result_open_current =
        if current_valve.flow_rate > 0 && !open_valves.contains(current_location) {
            let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
            new_open_valves.insert(current_location.to_string());

            let new_score = current_score + flow_rate;
            let new_flow_rate = flow_rate + current_valve.flow_rate;
            dfs_helper(
                minute + 1,
                current_location,
                new_flow_rate,
                new_score,
                &new_open_valves,
                valve_lookup,
                cache,
            )
        } else {
            None
        };

    let best_result_down_tunnels = current_valve
        .tunnels
        .iter()
        .filter_map(|next_valve_name| {
            dfs_helper(
                minute + 1,
                next_valve_name,
                flow_rate,
                current_score + flow_rate,
                open_valves,
                valve_lookup,
                cache,
            )
        })
        .max();

    best_result_down_tunnels.max(best_result_open_current)
}

#[allow(clippy::too_many_arguments)]
fn dfs_helper_2(
    minute: u32,
    my_location: &str,
    elephant_location: &str,
    flow_rate: u64,
    current_score: u64,
    open_valves: &HashSet<String>,
    valve_lookup: &HashMap<String, Valve>,
    cache: &mut HashMap<(u32, String, String, u64), u64>,
) -> Option<u64> {
    if minute > 26 {
        return Some(current_score);
    }

    let cache_key = (
        minute,
        my_location.to_string(),
        elephant_location.to_string(),
        flow_rate,
    );
    if let Some(cached_value) = cache.get(&cache_key) {
        if *cached_value >= current_score {
            return None;
        }
    }
    cache.insert(cache_key, current_score);

    let (my_flow_rate, my_tunnels) = {
        let valve = valve_lookup.get(my_location).unwrap();
        (valve.flow_rate, valve.tunnels.to_vec())
    };
    let (elephant_flow_rate, elephant_tunnels) = {
        let valve = valve_lookup.get(elephant_location).unwrap();
        (valve.flow_rate, valve.tunnels.to_vec())
    };

    let can_open_my_valve = my_flow_rate > 0 && !open_valves.contains(my_location);
    let can_open_elephant_valve =
        elephant_flow_rate > 0 && !open_valves.contains(elephant_location);
    let mut results = Vec::new();

    if can_open_my_valve {
        // open my valve, elephant moves
        let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(my_location.to_string());

        for new_elephant_location in elephant_tunnels.iter() {
            results.push(dfs_helper_2(
                minute + 1,
                my_location,
                new_elephant_location,
                flow_rate + my_flow_rate,
                current_score + flow_rate,
                &new_open_valves,
                valve_lookup,
                cache,
            ));
        }
    }

    if can_open_elephant_valve {
        // open elephant valve, i move
        let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(elephant_location.to_string());

        for new_my_location in my_tunnels.iter() {
            results.push(dfs_helper_2(
                minute + 1,
                new_my_location,
                elephant_location,
                flow_rate + elephant_flow_rate,
                current_score + flow_rate,
                &new_open_valves,
                valve_lookup,
                cache,
            ));
        }
    }

    if can_open_elephant_valve && can_open_my_valve && my_location != elephant_location {
        // elephant and i open our valves
        let mut new_open_valves = open_valves.iter().cloned().collect::<HashSet<_>>();
        new_open_valves.insert(elephant_location.to_string());
        new_open_valves.insert(my_location.to_string());

        results.push(dfs_helper_2(
            minute + 1,
            my_location,
            elephant_location,
            flow_rate + my_flow_rate + elephant_flow_rate,
            current_score + flow_rate,
            &new_open_valves,
            valve_lookup,
            cache,
        ));
    }

    // both elephant and i move
    for new_elephant_location in elephant_tunnels.iter() {
        for new_my_location in my_tunnels.iter() {
            results.push(dfs_helper_2(
                minute + 1,
                new_my_location,
                new_elephant_location,
                flow_rate,
                current_score + flow_rate,
                open_valves,
                valve_lookup,
                cache,
            ));
        }
    }

    results.into_iter().flatten().max()
}
