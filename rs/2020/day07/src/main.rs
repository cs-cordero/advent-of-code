use advent_of_code::*;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Bag {
    name: String,
    contained_by: HashMap<String, Rc<RefCell<Bag>>>,
    contains: HashMap<String, (usize, Weak<RefCell<Bag>>)>,
}

impl Bag {
    fn new_ref(name: String) -> Rc<RefCell<Bag>> {
        Rc::new(RefCell::new(Self {
            name,
            contained_by: HashMap::new(),
            contains: HashMap::new(),
        }))
    }

    fn add_contained_by(&mut self, bag: Rc<RefCell<Bag>>) {
        let name = bag.borrow().name.clone();
        self.contained_by.insert(name, bag);
    }

    fn add_contains(&mut self, count: usize, bag: Weak<RefCell<Bag>>) {
        if let Some(valid_bag) = bag.upgrade() {
            self.contains
                .insert(valid_bag.borrow().name.clone(), (count, bag));
        }
    }
}

fn main() {
    let graph = build_graph(&read_input_as_lines("2020/day07/src/input.txt"));
    let answer1 = count_unique_bags_which_hold_target_bag(&graph, String::from("shiny gold"));
    let answer2 =
        count_bags_recursively(&graph, Some(Rc::clone(graph.get("shiny gold").unwrap()))) - 1; // -1 to skip counting the shiny bag itself
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn build_graph(lines: &[String]) -> HashMap<String, Rc<RefCell<Bag>>> {
    let mut graph = HashMap::new();
    for line in lines {
        let (parent, children) = {
            let (left, right) = split_once_from_left(line, " contain ");

            let parent = {
                let (s, _) = split_once_from_right(left, " ");
                s.to_owned()
            };

            let children = right
                .trim_end_matches('.')
                .split(", ")
                .map(|child| {
                    let first_space = child.find(' ').unwrap();
                    let quantity = child[..first_space].parse::<usize>().unwrap_or(0);
                    let description = {
                        let (s, _) = split_once_from_right(&child[first_space + 1..], " ");
                        s.to_owned()
                    };
                    (quantity, description)
                })
                .filter(|(quantity, _)| quantity > &0)
                .collect::<Vec<_>>();

            (parent, children)
        };

        let parent_node = Rc::clone(
            graph
                .entry(parent.clone())
                .or_insert_with(|| Bag::new_ref(parent.clone())),
        );
        for (child_count, child) in children {
            let child_node = graph
                .entry(child.clone())
                .or_insert_with(|| Bag::new_ref(child.clone()));
            child_node
                .borrow_mut()
                .add_contained_by(Rc::clone(&parent_node));
            parent_node
                .borrow_mut()
                .add_contains(child_count, Rc::downgrade(child_node))
        }
    }
    graph
}

fn count_unique_bags_which_hold_target_bag(
    graph: &HashMap<String, Rc<RefCell<Bag>>>,
    target_bag_name: String,
) -> usize {
    let mut bags_holding_target = HashSet::<String>::new();
    let mut queue = VecDeque::<String>::new();
    queue.push_back(target_bag_name);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        graph
            .get(&current)
            .unwrap_or_else(|| panic!("Could not find node for [{}]", current))
            .borrow()
            .contained_by
            .keys()
            .for_each(|parent| {
                if !bags_holding_target.contains(parent) {
                    bags_holding_target.insert(parent.clone());
                    queue.push_back(parent.clone());
                }
            })
    }

    bags_holding_target.len()
}

fn count_bags_recursively(
    graph: &HashMap<String, Rc<RefCell<Bag>>>,
    maybe_bag: Option<Rc<RefCell<Bag>>>,
) -> usize {
    if let Some(bag) = maybe_bag {
        let bag_name = &bag.borrow().name;
        1 + graph
            .get(bag_name)
            .unwrap_or_else(|| panic!("Could not find node for [{}]", bag_name))
            .borrow()
            .contains
            .values()
            .map(|(count, child)| count * count_bags_recursively(graph, child.upgrade()))
            .sum::<usize>()
    } else {
        0
    }
}
