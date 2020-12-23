use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub value: u32,
    pub next: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn get_next(&self) -> Rc<RefCell<Node>> {
        self.next.as_ref().unwrap().upgrade().unwrap()
    }
}

#[allow(clippy::same_item_push)]
fn main() {
    let input = "925176834"
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    let answer1 = {
        let cups = build_linked_list(input.iter().copied());

        let mut current_cup = Rc::clone(cups.get(input.get(0).unwrap()).unwrap());
        for _ in 0..100 {
            current_cup = play(&cups, current_cup, 9);
        }

        let mut result = Vec::with_capacity(9);
        let mut curr = cups.get(&1).unwrap().borrow().get_next();
        for _ in 0..8 {
            result.push((curr.borrow().value as u32).to_string());
            let next = Rc::clone(&curr.borrow().get_next());
            curr = next;
        }
        result.join("")
    };

    let answer2 = {
        let cups = build_linked_list(input.iter().copied().chain(10..=1000000));

        let mut current_cup = Rc::clone(cups.get(input.get(0).unwrap()).unwrap());
        for _ in 0..10000000 {
            current_cup = play(&cups, current_cup, 1000000);
        }

        let node1 = Rc::clone(cups.get(&1).unwrap());
        let after_node1 = node1.borrow().get_next();
        let after_node2 = after_node1.borrow().get_next();
        let result = after_node1.borrow().value as u64 * after_node2.borrow().value as u64;
        result
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn play(
    nodes: &HashMap<u32, Rc<RefCell<Node>>>,
    current: Rc<RefCell<Node>>,
    max_value: i32,
) -> Rc<RefCell<Node>> {
    let next1 = current.borrow().get_next();
    let next2 = next1.borrow().get_next();
    let next3 = next2.borrow().get_next();
    let target = {
        let mut result = current.borrow().value as i32;
        while result == current.borrow().value as i32
            || result == next1.borrow().value as i32
            || result == next2.borrow().value as i32
            || result == next3.borrow().value as i32
        {
            result -= 1;
            if result == 0 {
                result = max_value;
            }
        }
        Rc::clone(nodes.get(&(result as u32)).unwrap())
    };
    current.borrow_mut().next = Some(Weak::clone(&next3.borrow().next.as_ref().unwrap()));
    next3.borrow_mut().next = Some(Weak::clone(&target.borrow().next.as_ref().unwrap()));
    target.borrow_mut().next = Some(Rc::downgrade(&next1));
    current.borrow().get_next()
}

fn build_linked_list<T>(input: T) -> HashMap<u32, Rc<RefCell<Node>>>
where
    T: Iterator<Item = u32>,
{
    let mut container: HashMap<u32, Rc<RefCell<Node>>> = HashMap::new();

    let fake_head = Rc::new(RefCell::new(Node {
        value: 0,
        next: None,
    }));
    let mut prev = Rc::clone(&fake_head);

    for value in input {
        let node = Rc::new(RefCell::new(Node { value, next: None }));
        prev.borrow_mut().next = Some(Rc::downgrade(&node));
        prev = Rc::clone(&node);
        container.insert(value, node);
    }

    let first = fake_head.borrow().get_next();
    let last = prev;
    last.borrow_mut().next = Some(Rc::downgrade(&first));

    container
}
