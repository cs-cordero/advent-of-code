extern crate core;

use std::collections::LinkedList;
use std::rc::Rc;
use advent_of_code::*;

fn main() {
    let data = read_input_as_lines("2022/day20/src/input.txt")
        .into_iter()
        .map(|line| Rc::new(line.parse::<i64>().unwrap()))
        .collect::<LinkedList<_>>();

    let solution1 = {
        let mut data = data.iter().cloned().collect::<LinkedList<_>>();
        let order = data.iter().cloned().collect::<Vec<_>>();
        mix(&mut data, &order);
        find_answer(&data)
    };

    let solution2 = {
        let decryption_key: i64 = 811589153;
        let mut data = data
            .iter()
            .map(|rc| **rc)
            .map(|value| Rc::new(value * decryption_key))
            .collect::<LinkedList<_>>();
        let order = data.iter().cloned().collect::<Vec<_>>();

        for _ in 0..10 {
            mix(&mut data, &order);
        }

        find_answer(&data)
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn find(list: &LinkedList<Rc<i64>>, target: &Rc<i64>) -> Option<usize> {
    for (i, rc) in list.iter().enumerate() {
        if Rc::ptr_eq(rc, target) {
            return Some(i);
        }
    }

    None
}

fn get_new_index(index: usize, delta: i64, list_length: usize) -> usize {
    (index as isize + delta as isize).rem_euclid(list_length as isize - 1) as usize
    // let mut new_index = index as isize + delta as isize;
    // if delta < 0 {
    //     new_index -= 1;
    // }
    //
    // while new_index < 0 {
    //     new_index += list_length as isize;
    // }
    //
    // let new_index = new_index as usize;
    // if new_index > list_length {
    //     (new_index % list_length) + 1
    // } else if new_index == list_length {
    //     0
    // } else {
    //     new_index
    // }
}

fn remove_at<T>(list: &mut LinkedList<T>, at: usize) {
    let mut split = list.split_off(at);
    split.pop_front();
    list.append(&mut split);
}

fn mix(list: &mut LinkedList<Rc<i64>>, order: &[Rc<i64>]) {
    for rc in order {
        let index = find(list, rc).unwrap();
        let new_index = get_new_index(index, **rc, list.len());

        remove_at(list, index);
        let mut latter = list.split_off(new_index);
        latter.push_front(Rc::clone(rc));
        list.append(&mut latter);
    }
}

fn find_answer(list: &LinkedList<Rc<i64>>) -> i64 {
    let vec = list.iter().collect::<Vec<_>>();
    let zero_index = vec.iter().enumerate().find(|(_, &rc)| (**rc) == 0).map(|(i, _)| i).unwrap();

    let num1 = ***vec.get((zero_index + 1000) % vec.len()).unwrap();
    let num2 = ***vec.get((zero_index + 2000) % vec.len()).unwrap();
    let num3 = ***vec.get((zero_index + 3000) % vec.len()).unwrap();

    num1 + num2 + num3
}
