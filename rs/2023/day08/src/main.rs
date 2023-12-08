use std::collections::{HashMap, HashSet};

use advent_of_code::*;

struct Edges {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Cycle {
    start: u32,
    size: u32,
    elements: Vec<String>,
}

fn main() {
    let (steps, graph) = {
        let raw = read_input_as_string("2023/day08/src/input.txt");
        let (steps, raw_graph) = raw.split_once("\n\n").unwrap();
        let steps = steps.to_string();
        let mut graph = HashMap::new();

        for line in raw_graph.lines() {
            let (source, targets) = line.split_once(" = (").unwrap();
            let targets = targets.strip_suffix(')').unwrap();
            let (target_left, target_right) = targets.split_once(", ").unwrap();
            let edges = Edges {
                left: target_left.to_string(),
                right: target_right.to_string(),
            };

            graph.insert(source.to_string(), edges);
        }

        (steps, graph)
    };

    let part1 = {
        let mut current_location = "AAA";
        let mut repeated_steps = steps.chars().cycle();
        let mut step_count = 0;
        while let Some(step) = repeated_steps.next() {
            if current_location == "ZZZ" {
                break;
            }

            let edges = graph
                .get(current_location)
                .expect(&format!("Did not find an edge for {}", current_location));
            let next_location = match step {
                'L' => &edges.left,
                'R' => &edges.right,
                _ => panic!("Invalid step"),
            };

            current_location = next_location;
            step_count += 1;
        }
        step_count
    };

    let part2 = {
        let mut cycles = Vec::new();
        let current_locations = graph
            .keys()
            .filter(|location| location.ends_with('A'))
            .map(String::as_str)
            .collect::<Vec<_>>();

        for loc in current_locations {
            cycles.push((loc, determine_cycle(&steps, &graph, loc)));
        }

        cycles
            .iter()
            .map(|(_, cycle)| cycle.size as u64)
            .reduce(|acc, el| lcm(acc, el))
            .unwrap()
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

fn determine_cycle(all_directions: &str, graph: &HashMap<String, Edges>, start: &str) -> Cycle {
    let mut current_location = start;
    let mut step_count = 0;
    let mut seen = HashSet::new();

    let mut directions = all_directions.chars().enumerate().cycle();
    let cycle_first = loop {
        let (i, direction) = directions.next().unwrap();
        step(direction, graph, &mut current_location, &mut step_count);

        let key = (current_location, i);
        if !seen.insert(key) {
            break key;
        }
    };

    let cycle_start = step_count;
    let mut elements = Vec::new();
    elements.push(current_location);
    step_count = 0;

    for (i, direction) in directions {
        step(direction, graph, &mut current_location, &mut step_count);

        let key = (current_location, i);
        if key == cycle_first {
            break;
        }

        elements.push(current_location);
    }

    assert_eq!(step_count, elements.len());

    Cycle {
        start: cycle_start as u32,
        size: step_count as u32,
        elements: elements.into_iter().map(|s| s.to_owned()).collect(),
    }
}

fn step<'a, 'b: 'a>(
    direction: char,
    graph: &'b HashMap<String, Edges>,
    current_location: &'a mut &'b str,
    step_count: &'a mut usize,
) {
    let edges = graph.get(*current_location).unwrap();
    let next_location = match direction {
        'L' => &edges.left,
        'R' => &edges.right,
        _ => panic!("Invalid step"),
    };

    *current_location = next_location;
    *step_count += 1;
}
