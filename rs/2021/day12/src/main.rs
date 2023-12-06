use advent_of_code::*;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum CaveType {
    Big,
    Small,
    Source,
    Sink,
}

impl FromStr for CaveType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(CaveType::Source),
            "end" => Ok(CaveType::Sink),
            _ => {
                if s.chars().next().unwrap().is_uppercase() {
                    Ok(CaveType::Big)
                } else {
                    Ok(CaveType::Small)
                }
            }
        }
    }
}

#[derive(Debug)]
struct Cave {
    cave_type: CaveType,
    paths: Vec<String>,
}

fn main() {
    let graph = {
        let mut result = HashMap::<String, Cave>::new();
        for line in read_input_as_lines("2021/day12/src/input.txt") {
            let (source, target) = line.split_once('-').unwrap();
            let source = source.to_string();
            let target = target.to_string();
            {
                let cave_type = CaveType::from_str(&source).unwrap();
                let entry = result.entry(source.clone()).or_insert_with(|| Cave {
                    cave_type,
                    paths: Vec::new(),
                });
                entry.paths.push(target.clone());
            }
            {
                let cave_type = CaveType::from_str(&target).unwrap();
                let entry = result.entry(target).or_insert_with(|| Cave {
                    cave_type,
                    paths: Vec::new(),
                });
                entry.paths.push(source);
            }
        }
        result
    };

    let answer1 = dfs("start", &graph, &mut HashMap::new(), false);
    let answer2 = dfs("start", &graph, &mut HashMap::new(), true);

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

fn dfs(
    current: &str,
    graph: &HashMap<String, Cave>,
    visited: &mut HashMap<String, usize>,
    can_revisit_small_cave: bool,
) -> usize {
    if current == "end" {
        return 1;
    } else if current == "start" && visited.contains_key("start") {
        return 0;
    }

    let cave = graph.get(current).expect("Missing an expected cave.");
    let visit_count = if let Some(entry) = visited.get_mut(current) {
        *entry += 1;
        *entry
    } else {
        visited.insert(current.to_string(), 1);
        1
    };

    if visit_count > 100
        || (visit_count > 1 && cave.cave_type != CaveType::Big && !can_revisit_small_cave)
    {
        if let Some(entry) = visited.get_mut(current) {
            *entry = entry.saturating_sub(1);
        }
        return 0;
    }

    let can_revisit_small_cave =
        can_revisit_small_cave && (cave.cave_type != CaveType::Small || visit_count <= 1);

    let mut result = 0;
    for next_cave in cave.paths.iter() {
        result += dfs(next_cave.as_str(), graph, visited, can_revisit_small_cave);
    }

    if let Some(entry) = visited.get_mut(current) {
        *entry = entry.saturating_sub(1);
    }

    result
}
