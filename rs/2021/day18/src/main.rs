use advent_of_code::*;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::VecDeque;

type SnailfishIndex = usize;

#[derive(Debug)]
enum Snailfish {
    Number {
        id: SnailfishIndex,
        value: u32,
    },
    Pair {
        id: SnailfishIndex,
        left: SnailfishIndex,
        right: SnailfishIndex,
    },
}

impl Snailfish {
    fn id(&self) -> SnailfishIndex {
        match self {
            Snailfish::Number { id, .. } => *id,
            Snailfish::Pair { id, .. } => *id,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Snailfish::Number { value, .. } => *value,
            Snailfish::Pair { .. } => panic!("Snailfish pairs don't have a value."),
        }
    }
}

fn main() {
    let lines = read_input_as_lines("2021/day18/src/input.txt");

    let answer1 = {
        let (first_line, lines) = lines.split_at(1);
        let (mut root, mut graph) = parse_graph(first_line[0].chars().collect());
        for line in lines {
            let (rhs_root, rhs_graph) = parse_graph(line.chars().collect());
            root = add_snailfish(&mut graph, rhs_graph, root, rhs_root);
            reduce_snailfish(&mut graph, root);
        }

        find_magnitude(&graph, root)
    };

    let answer2 = {
        let mut result = 0;
        for lhs in 0..lines.len() - 1 {
            for rhs in lhs + 1..lines.len() {
                unsafe {
                    reset_id_generator();
                }
                let (l_root, mut l_graph) = parse_graph(lines[lhs].chars().collect());
                let (r_root, r_graph) = parse_graph(lines[rhs].chars().collect());
                let root = add_snailfish(&mut l_graph, r_graph, l_root, r_root);
                reduce_snailfish(&mut l_graph, root);
                result = max(result, find_magnitude(&l_graph, root));

                unsafe {
                    reset_id_generator();
                }
                let (l_root, l_graph) = parse_graph(lines[lhs].chars().collect());
                let (r_root, mut r_graph) = parse_graph(lines[rhs].chars().collect());
                let root = add_snailfish(&mut r_graph, l_graph, r_root, l_root);
                reduce_snailfish(&mut r_graph, root);
                result = max(result, find_magnitude(&r_graph, root));
            }
        }
        result
    };

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

static mut ID_GENERATOR: RefCell<usize> = RefCell::new(0);

unsafe fn generate_id() -> usize {
    let generator = ID_GENERATOR.get_mut();
    let id = *generator;
    *generator += 1;
    id
}

unsafe fn reset_id_generator() {
    *ID_GENERATOR.get_mut() = 0;
}

fn add_snailfish(
    left_graph: &mut Vec<Snailfish>,
    right_graph: Vec<Snailfish>,
    left_root: SnailfishIndex,
    right_root: SnailfishIndex,
) -> SnailfishIndex {
    let new_id = unsafe { generate_id() };
    let new_root = Snailfish::Pair {
        id: new_id,
        left: left_root,
        right: right_root,
    };
    left_graph.extend(right_graph);
    left_graph.push(new_root);
    left_graph.sort_by_key(|node| node.id());
    new_id
}

fn reduce_snailfish(graph: &mut Vec<Snailfish>, root: SnailfishIndex) {
    loop {
        if let Some(exploder_index) = locate_id_that_needs_to_explode(graph, root, 0) {
            let explode_config = get_explode_configuration(graph, root, exploder_index);
            explode(
                graph,
                explode_config.pair_exploder,
                explode_config.number_left,
                explode_config.number_right,
            );
        } else if let Some(splitter_index) = locate_id_that_needs_to_split(graph, root) {
            split_snailfish(graph, splitter_index);
        } else {
            break;
        }
    }
}

/// [[[[4,3],4],4],[7,[[8,4],9]]]
fn parse_graph(mut s: VecDeque<char>) -> (SnailfishIndex, Vec<Snailfish>) {
    let mut result: Vec<Snailfish> = Vec::new();

    fn helper<'a>(
        remaining: &'a mut VecDeque<char>,
        result: &'a mut Vec<Snailfish>,
    ) -> Option<SnailfishIndex> {
        if let Some(next_char) = remaining.pop_front() {
            if next_char == '[' {
                let left = helper(remaining, result).unwrap();
                let right = helper(remaining, result).unwrap();
                let snailfish = Snailfish::Pair {
                    id: unsafe { generate_id() },
                    left,
                    right,
                };
                result.push(snailfish);

                if !remaining.is_empty() && (remaining[0] == ',' || remaining[0] == ']') {
                    remaining.pop_front();
                }

                result.last().map(|snailfish| snailfish.id())
            } else if next_char.is_digit(10) {
                let mut value = next_char.to_digit(10).unwrap();
                loop {
                    let next_char = remaining.pop_front().unwrap();
                    if let Some(digit) = next_char.to_digit(10) {
                        value *= 10;
                        value += digit;
                    } else if next_char == ',' || next_char == ']' {
                        break;
                    } else {
                        panic!("Unexpected character {}", next_char)
                    }
                }
                let snailfish = Snailfish::Number {
                    id: unsafe { generate_id() },
                    value,
                };
                result.push(snailfish);
                result.last().map(|snailfish| snailfish.id())
            } else {
                panic!(
                    "Unexpected character {}, remaining: {:?}",
                    next_char, remaining
                )
            }
        } else {
            None
        }
    }

    let root = helper(&mut s, &mut result).unwrap();
    result.sort_by_key(|node| node.id());
    (root, result)
}

/// Find the leftmost pair of regular numbers that is at depth >= 4
fn locate_id_that_needs_to_explode(
    graph: &[Snailfish],
    root: SnailfishIndex,
    depth: usize,
) -> Option<SnailfishIndex> {
    let node = graph.get(root).unwrap();
    let is_pair_with_regular_numbers = match node {
        Snailfish::Number { .. } => false,
        Snailfish::Pair { left, right, .. } => {
            matches!(graph.get(*left).unwrap(), Snailfish::Number { .. })
                && matches!(graph.get(*right).unwrap(), Snailfish::Number { .. })
        }
    };
    if depth >= 4 && is_pair_with_regular_numbers {
        Some(node.id())
    } else {
        match node {
            Snailfish::Pair { left, right, .. } => {
                locate_id_that_needs_to_explode(graph, *left, depth + 1)
                    .or_else(|| locate_id_that_needs_to_explode(graph, *right, depth + 1))
            }
            _ => None,
        }
    }
}

/// Find the leftmost regular number that has value >= 10
fn locate_id_that_needs_to_split(
    graph: &[Snailfish],
    root: SnailfishIndex,
) -> Option<SnailfishIndex> {
    let node = graph.get(root).unwrap();
    match node {
        Snailfish::Number { value, .. } => {
            if *value >= 10 {
                Some(node.id())
            } else {
                None
            }
        }
        Snailfish::Pair { left, right, .. } => locate_id_that_needs_to_split(graph, *left)
            .or_else(|| locate_id_that_needs_to_split(graph, *right)),
    }
}

/// Only gets the inorder traversals of the leaf nodes
fn get_leaf_inorder_traversal(graph: &[Snailfish], root: SnailfishIndex) -> Vec<SnailfishIndex> {
    let mut result = Vec::new();
    fn helper(graph: &[Snailfish], node_index: SnailfishIndex, result: &mut Vec<SnailfishIndex>) {
        let node = graph.get(node_index).unwrap();
        match node {
            Snailfish::Pair { left, right, .. } => {
                helper(graph, *left, result);
                helper(graph, *right, result);
            }
            Snailfish::Number { id, .. } => result.push(*id),
        }
    }
    helper(graph, root, &mut result);
    result
}

#[derive(Debug)]
struct ExplodeConfiguration {
    pair_exploder: SnailfishIndex,
    number_left: Option<SnailfishIndex>,
    number_right: Option<SnailfishIndex>,
}

fn get_explode_configuration(
    graph: &[Snailfish],
    root: SnailfishIndex,
    exploder_index: SnailfishIndex,
) -> ExplodeConfiguration {
    let (pair_left_id, pair_right_id) = {
        match graph.get(exploder_index).unwrap() {
            Snailfish::Number { .. } => panic!("No"),
            Snailfish::Pair { left, right, .. } => (*left, *right),
        }
    };

    let inorder_traversal = get_leaf_inorder_traversal(graph, root);

    let number_left = inorder_traversal
        .iter()
        .position(|id| *id == pair_left_id)
        .unwrap()
        .checked_sub(1)
        .map(|i| *inorder_traversal.get(i).unwrap());
    let number_right = inorder_traversal
        .iter()
        .position(|id| *id == pair_right_id)
        .map(|i| i + 1)
        .filter(|i| *i < inorder_traversal.len())
        .map(|i| *inorder_traversal.get(i).unwrap());
    ExplodeConfiguration {
        pair_exploder: exploder_index,
        number_left,
        number_right,
    }
}

/// exploder is expected to be a SnailfishIndex of a Pair.
/// left and right is expected to be a SnailfishIndex of a Number ONLY.
fn explode(
    graph: &mut Vec<Snailfish>,
    exploder_index: SnailfishIndex,
    left: Option<SnailfishIndex>,
    right: Option<SnailfishIndex>,
) {
    let exploder = graph.get_mut(exploder_index).unwrap();
    let dull_node = Snailfish::Number {
        id: exploder.id(),
        value: 0,
    };
    let exploder = std::mem::replace(exploder, dull_node);

    let (exploded_left_value, exploded_right_value) = match exploder {
        Snailfish::Pair { left, right, .. } => (
            graph.get(left).unwrap().value(),
            graph.get(right).unwrap().value(),
        ),
        _ => panic!("Only pairs should explode"),
    };

    if let Some(left) = left {
        let leaf_node = graph.get_mut(left).unwrap();
        match leaf_node {
            Snailfish::Number { value, .. } => *value += exploded_left_value,
            _ => panic!("Nope"),
        }
    }

    if let Some(right) = right {
        let leaf_node = graph.get_mut(right).unwrap();
        match leaf_node {
            Snailfish::Number { value, .. } => *value += exploded_right_value,
            _ => panic!("Nope"),
        }
    }
}

fn split_snailfish(graph: &mut Vec<Snailfish>, split_index: SnailfishIndex) {
    let splitter = graph.get_mut(split_index).unwrap();
    let value = splitter.value();
    let new_left = Snailfish::Number {
        id: unsafe { generate_id() },
        value: (value as f32 / 2.0).floor() as u32,
    };
    let new_right = Snailfish::Number {
        id: unsafe { generate_id() },
        value: (value as f32 / 2.0).ceil() as u32,
    };
    let new_pair = Snailfish::Pair {
        id: splitter.id(),
        left: new_left.id(),
        right: new_right.id(),
    };
    graph.push(new_left);
    graph.push(new_right);
    graph[split_index] = new_pair;
}

fn find_magnitude(graph: &[Snailfish], root: SnailfishIndex) -> u32 {
    let snailfish = graph.get(root).unwrap();
    match snailfish {
        Snailfish::Number { value, .. } => *value,
        Snailfish::Pair { left, right, .. } => {
            let left_magnitude = find_magnitude(graph, *left);
            let right_magnitude = find_magnitude(graph, *right);
            3 * left_magnitude + 2 * right_magnitude
        }
    }
}
