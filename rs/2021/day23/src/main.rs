use advent_of_code::{get_adjacent_points_manhattan, get_limits, read_input_as_string};
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

type Point = (usize, usize);
static TEMP_STORAGE_COORDINATES: [Point; 7] =
    [(1, 1), (1, 2), (1, 4), (1, 6), (1, 8), (1, 10), (1, 11)];
static PART_1_TARGET_DESTINATION_A: [Point; 2] = [(2, 3), (3, 3)];
static PART_1_TARGET_DESTINATION_B: [Point; 2] = [(2, 5), (3, 5)];
static PART_1_TARGET_DESTINATION_C: [Point; 2] = [(2, 7), (3, 7)];
static PART_1_TARGET_DESTINATION_D: [Point; 2] = [(2, 9), (3, 9)];
static PART_2_TARGET_DESTINATION_A: [Point; 4] = [(2, 3), (3, 3), (4, 3), (5, 3)];
static PART_2_TARGET_DESTINATION_B: [Point; 4] = [(2, 5), (3, 5), (4, 5), (5, 5)];
static PART_2_TARGET_DESTINATION_C: [Point; 4] = [(2, 7), (3, 7), (4, 7), (5, 7)];
static PART_2_TARGET_DESTINATION_D: [Point; 4] = [(2, 9), (3, 9), (4, 9), (5, 9)];

#[derive(Copy, Clone, Debug, PartialEq)]
enum Amphipod {
    A,
    B,
    C,
    D,
    Wall,
    Empty,
}

impl FromStr for Amphipod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "." => Ok(Self::Empty),
            " " | "#" => Ok(Self::Wall),
            _ => Err(format!("Could not parse {} into an Amphipod", s)),
        }
    }
}

impl Amphipod {
    fn serialize(&self) -> String {
        match self {
            Amphipod::A => "A",
            Amphipod::B => "B",
            Amphipod::C => "C",
            Amphipod::D => "D",
            Amphipod::Wall => "#",
            Amphipod::Empty => ".",
        }
        .to_string()
    }
}

#[inline]
fn serialize(graph: &[Vec<Amphipod>]) -> String {
    graph
        .iter()
        .map(|line| {
            line.iter()
                .map(|a| a.serialize())
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("")
}

#[inline]
fn deepcopy(graph: &[Vec<Amphipod>]) -> Vec<Vec<Amphipod>> {
    graph.to_vec()
}

/// Optionally returns the cost of traveling to the target point.
/// Returns None if the point cannot be reached.
fn bfs(graph: &[Vec<Amphipod>], start: Point, target: Point) -> Option<usize> {
    let limits = get_limits(graph);
    let cost_per_move = match graph.get(start.0).unwrap().get(start.1).unwrap() {
        Amphipod::A => 1,
        Amphipod::B => 10,
        Amphipod::C => 100,
        Amphipod::D => 1000,
        _ => panic!("Only Amphipods may move. Tried to move from {:?}", start),
    };

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    for point in get_adjacent_points_manhattan(start, limits) {
        queue.push_back((point, 1));
        visited.insert(point);
    }

    while let Some((coordinate, steps)) = queue.pop_front() {
        let current = graph.get(coordinate.0).unwrap().get(coordinate.1).unwrap();

        if !matches!(current, Amphipod::Empty) {
            // We can only traverse empty regions.
            continue;
        } else if coordinate == target {
            return Some(steps * cost_per_move);
        }

        for next_point in get_adjacent_points_manhattan(coordinate, limits) {
            if visited.contains(&next_point) {
                continue;
            } else {
                visited.insert(next_point);
                queue.push_back((next_point, steps + 1))
            }
        }
    }

    None
}

fn get_possible_moves(graph: &[Vec<Amphipod>]) -> Vec<(Point, Point)> {
    let source_movers = std::iter::once((PART_1_TARGET_DESTINATION_A, Amphipod::A))
        .chain(std::iter::once((PART_1_TARGET_DESTINATION_B, Amphipod::B)))
        .chain(std::iter::once((PART_1_TARGET_DESTINATION_C, Amphipod::C)))
        .chain(std::iter::once((PART_1_TARGET_DESTINATION_D, Amphipod::D)))
        .filter_map(|(locations, target_amphipod)| {
            let [first, second] = locations;
            let first_amphipod = get(graph, first).unwrap();
            let second_amphipod = get(graph, second).unwrap();
            if [first_amphipod, second_amphipod]
                .iter()
                .all(|a| *a == Amphipod::Empty || *a == target_amphipod)
            {
                // already solved or completely empty
                None
            } else if first_amphipod == Amphipod::Empty {
                if second_amphipod != target_amphipod {
                    Some(second)
                } else {
                    None
                }
            } else if second_amphipod != target_amphipod || first_amphipod != target_amphipod {
                // the deeper spot doesn't match, so even if the first spot matches, it needs to get out of the way.
                Some(first)
            } else {
                unreachable!()
            }
        })
        .flat_map(|source_point| {
            TEMP_STORAGE_COORDINATES
                .iter()
                .filter_map(
                    move |target_point| match get(graph, *target_point).unwrap() {
                        Amphipod::Empty => Some((source_point, *target_point)),
                        _ => None,
                    },
                )
        });

    let storage_movers = TEMP_STORAGE_COORDINATES.iter().filter_map(|&point| {
        let (target_destination, amphipod_type) = match get(graph, point).unwrap() {
            Amphipod::A => (PART_1_TARGET_DESTINATION_A, Amphipod::A),
            Amphipod::B => (PART_1_TARGET_DESTINATION_B, Amphipod::B),
            Amphipod::C => (PART_1_TARGET_DESTINATION_C, Amphipod::C),
            Amphipod::D => (PART_1_TARGET_DESTINATION_D, Amphipod::D),
            _ => {
                return None;
            }
        };
        let [first, second] = target_destination;
        let first_amphipod = get(graph, first).unwrap();
        let second_amphipod = get(graph, second).unwrap();
        if matches!(first_amphipod, Amphipod::Empty) && matches!(second_amphipod, Amphipod::Empty) {
            Some((point, second))
        } else if matches!(first_amphipod, Amphipod::Empty)
            && ((amphipod_type == Amphipod::A && matches!(second_amphipod, Amphipod::A))
                || (amphipod_type == Amphipod::B && matches!(second_amphipod, Amphipod::B))
                || (amphipod_type == Amphipod::C && matches!(second_amphipod, Amphipod::C))
                || (amphipod_type == Amphipod::D && matches!(second_amphipod, Amphipod::D)))
        {
            Some((point, first))
        } else {
            None
        }
    });

    source_movers.chain(storage_movers).collect()
}

fn get_possible_moves_part2(graph: &[Vec<Amphipod>]) -> Vec<(Point, Point)> {
    let source_movers = std::iter::once((PART_2_TARGET_DESTINATION_A, Amphipod::A))
        .chain(std::iter::once((PART_2_TARGET_DESTINATION_B, Amphipod::B)))
        .chain(std::iter::once((PART_2_TARGET_DESTINATION_C, Amphipod::C)))
        .chain(std::iter::once((PART_2_TARGET_DESTINATION_D, Amphipod::D)))
        .filter_map(|(locations, target_amphipod)| {
            let [first, second, third, fourth] = locations;
            let first_amphipod = get(graph, first).unwrap();
            let second_amphipod = get(graph, second).unwrap();
            let third_amphipod = get(graph, third).unwrap();
            let fourth_amphipod = get(graph, fourth).unwrap();

            let already_solved_or_all_empty = [
                first_amphipod,
                second_amphipod,
                third_amphipod,
                fourth_amphipod,
            ]
            .iter()
            .all(|a| *a == target_amphipod || *a == Amphipod::Empty);
            if already_solved_or_all_empty {
                None
            } else if [
                first_amphipod,
                second_amphipod,
                third_amphipod,
                fourth_amphipod,
            ]
            .iter()
            .any(|a| *a != target_amphipod)
                && first_amphipod != Amphipod::Empty
            {
                Some(first)
            } else if third_amphipod == Amphipod::Empty {
                if fourth_amphipod != target_amphipod && fourth_amphipod != Amphipod::Empty {
                    Some(fourth)
                } else {
                    None
                }
            } else if second_amphipod == Amphipod::Empty {
                if [third_amphipod, fourth_amphipod]
                    .iter()
                    .any(|a| *a != target_amphipod && *a != Amphipod::Empty)
                    && third_amphipod != Amphipod::Empty
                {
                    Some(third)
                } else {
                    None
                }
            } else if first_amphipod == Amphipod::Empty {
                if [second_amphipod, third_amphipod, fourth_amphipod]
                    .iter()
                    .any(|a| *a != target_amphipod && *a != Amphipod::Empty)
                    && second_amphipod != Amphipod::Empty
                {
                    Some(second)
                } else {
                    None
                }
            } else {
                unreachable!(
                    "Should not have reached here: (target: {:?}) {:?} {:?} {:?} {:?}",
                    target_amphipod,
                    first_amphipod,
                    second_amphipod,
                    third_amphipod,
                    fourth_amphipod
                );
            }
        })
        .flat_map(|source_point| {
            let target_temp_storage =
                TEMP_STORAGE_COORDINATES
                    .iter()
                    .filter_map(
                        move |target_point| match get(graph, *target_point).unwrap() {
                            Amphipod::Empty => Some((source_point, *target_point)),
                            _ => None,
                        },
                    );

            let destination_target = {
                let target_amphipod = get(graph, source_point).unwrap();
                let target_destinations = match target_amphipod {
                    Amphipod::A => PART_2_TARGET_DESTINATION_A,
                    Amphipod::B => PART_2_TARGET_DESTINATION_B,
                    Amphipod::C => PART_2_TARGET_DESTINATION_C,
                    Amphipod::D => PART_2_TARGET_DESTINATION_D,
                    _ => panic!(),
                };

                if !target_destinations
                    .iter()
                    .map(|t| get(graph, *t).unwrap())
                    .all(|a| a == Amphipod::Empty || a == target_amphipod)
                {
                    None
                } else {
                    target_destinations
                        .iter()
                        .filter(|&t| get(graph, *t).unwrap() == Amphipod::Empty)
                        .last()
                        .copied()
                        .map(|target_point| (source_point, target_point))
                }
            };

            std::iter::once(destination_target)
                .flatten()
                .chain(target_temp_storage)
        });

    let storage_movers = TEMP_STORAGE_COORDINATES.iter().filter_map(|&point| {
        let (target_destination, amphipod_type) = match get(graph, point).unwrap() {
            Amphipod::A => (PART_2_TARGET_DESTINATION_A, Amphipod::A),
            Amphipod::B => (PART_2_TARGET_DESTINATION_B, Amphipod::B),
            Amphipod::C => (PART_2_TARGET_DESTINATION_C, Amphipod::C),
            Amphipod::D => (PART_2_TARGET_DESTINATION_D, Amphipod::D),
            _ => {
                return None;
            }
        };
        let [first, second, third, fourth] = target_destination;
        let first_amphipod = get(graph, first).unwrap();
        let second_amphipod = get(graph, second).unwrap();
        let third_amphipod = get(graph, third).unwrap();
        let fourth_amphipod = get(graph, fourth).unwrap();
        if [
            first_amphipod,
            second_amphipod,
            third_amphipod,
            fourth_amphipod,
        ]
        .iter()
        .any(|a| *a != Amphipod::Empty && *a != amphipod_type)
        {
            None
        } else if [
            first_amphipod,
            second_amphipod,
            third_amphipod,
            fourth_amphipod,
        ]
        .iter()
        .all(|a| *a == Amphipod::Empty)
        {
            Some((point, fourth))
        } else if [first_amphipod, second_amphipod, third_amphipod]
            .iter()
            .all(|a| *a == Amphipod::Empty)
        {
            Some((point, third))
        } else if [first_amphipod, second_amphipod]
            .iter()
            .all(|a| *a == Amphipod::Empty)
        {
            Some((point, second))
        } else if [first_amphipod].iter().all(|a| *a == Amphipod::Empty) {
            Some((point, first))
        } else {
            None
        }
    });

    source_movers.chain(storage_movers).collect()
}

#[inline]
fn get(graph: &[Vec<Amphipod>], point: Point) -> Option<Amphipod> {
    graph.get(point.0).and_then(|r| r.get(point.1).copied())
}

fn solve_part_1(
    graph: Vec<Vec<Amphipod>>,
    current_cost: usize,
    distances_traveled: &mut HashMap<String, usize>,
) {
    for (source_point, target_point) in get_possible_moves(&graph) {
        // target can be reached
        if let Some(cost) = bfs(&graph, source_point, target_point) {
            let amphipod = get(&graph, source_point).unwrap();
            let mut new_graph = deepcopy(&graph.clone());
            let source = new_graph
                .get_mut(source_point.0)
                .and_then(|r| r.get_mut(source_point.1))
                .unwrap();
            *source = Amphipod::Empty;
            let target = new_graph
                .get_mut(target_point.0)
                .and_then(|r| r.get_mut(target_point.1))
                .unwrap();
            *target = amphipod;

            let key = serialize(&new_graph);
            let new_cost = current_cost + cost;

            // reached with a cheaper cost
            let entry = distances_traveled.entry(key).or_insert(usize::MAX);
            if new_cost < *entry {
                *entry = new_cost;
                solve_part_1(new_graph, new_cost, distances_traveled);
            }
        }
    }
}

fn solve_part_2(
    graph: Vec<Vec<Amphipod>>,
    current_cost: usize,
    distances_traveled: &mut HashMap<String, usize>,
) {
    for (source_point, target_point) in get_possible_moves_part2(&graph) {
        // target can be reached
        if let Some(cost) = bfs(&graph, source_point, target_point) {
            let amphipod = get(&graph, source_point).unwrap();
            let mut new_graph = deepcopy(&graph.clone());
            let source = new_graph
                .get_mut(source_point.0)
                .and_then(|r| r.get_mut(source_point.1))
                .unwrap();
            *source = Amphipod::Empty;
            let target = new_graph
                .get_mut(target_point.0)
                .and_then(|r| r.get_mut(target_point.1))
                .unwrap();
            *target = amphipod;

            let key = serialize(&new_graph);
            let new_cost = current_cost + cost;

            // reached with a cheaper cost
            let entry = distances_traveled.entry(key).or_insert(usize::MAX);
            if new_cost < *entry {
                *entry = new_cost;
                solve_part_2(new_graph, new_cost, distances_traveled);
            }
        }
    }
}

static TARGET_KEY: &str = "##############...........####A#B#C#D######A#B#C#D############";
static TARGET_KEY_PART_2: &str =
    "##############...........####A#B#C#D######A#B#C#D####A#B#C#D####A#B#C#D############";

fn main() {
    let graph = read_input_as_string("2021/day23/src/input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Amphipod::from_str(&c.to_string()).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answer1 = {
        let mut memo = HashMap::new();
        solve_part_1(deepcopy(&graph), 0, &mut memo);
        *memo.get(&TARGET_KEY.to_string()).unwrap()
    };

    let answer2 = {
        let mut memo = HashMap::new();
        let mut graph = deepcopy(&graph);
        graph.insert(
            3,
            "###D#C#B#A#"
                .chars()
                .map(|c| Amphipod::from_str(&c.to_string()).unwrap())
                .collect(),
        );
        graph.insert(
            4,
            "###D#B#A#C#"
                .chars()
                .map(|c| Amphipod::from_str(&c.to_string()).unwrap())
                .collect(),
        );
        solve_part_2(graph, 0, &mut memo);
        *memo.get(&TARGET_KEY_PART_2.to_string()).unwrap()
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}
