extern crate core;

use advent_of_code::*;

#[derive(Debug)]
enum Element {
    Value(u32),
    List(Vec<Element>),
}

fn main() {
    let solution1 = read_input_as_string("2022/day13/src/input.txt")
        .split("\n\n")
        .enumerate()
        .filter(|(_, s)| {
            let mut lines = s.lines();
            let left_line = lines.next().unwrap();
            let left = parse(&mut left_line.chars());

            let right_line = lines.next().unwrap();
            let right = parse(&mut right_line.chars());

            is_in_correct_order(&left, &right).unwrap_or(true)
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let solution2 = {
        let packets = read_input_as_string("2022/day13/src/input.txt")
            .split("\n\n")
            .flat_map(|chunk| chunk.lines())
            .map(|line| parse(&mut line.chars()))
            .collect::<Vec<_>>();

        let partition_element_1 = Element::List(vec![Element::List(vec![Element::Value(2)])]);
        let partition_element_2 = Element::List(vec![Element::List(vec![Element::Value(6)])]);

        let (part1, part2) = partition(packets, partition_element_1);
        let (part2, _) = partition(part2, partition_element_2);

        let divider_packet_1_index = part1.len() + 1;
        let divider_packet_2_index = part1.len() + part2.len() + 1;

        divider_packet_1_index * divider_packet_2_index
    };

    println!("Part 1: {:?}", solution1);
    println!("Part 2: {:?}", solution2);
}

fn parse(s: &mut impl Iterator<Item = char>) -> Element {
    let mut list: Vec<Element> = Vec::new();

    let mut temp_digit_store: Vec<char> = Vec::new();
    while let Some(char) = s.next() {
        if char.is_ascii_digit() {
            temp_digit_store.push(char);
            continue;
        } else if !temp_digit_store.is_empty() {
            let number = temp_digit_store
                .iter()
                .copied()
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
            temp_digit_store.clear();

            list.push(Element::Value(number));
        }

        match char {
            '[' => {
                let inner_element = parse(s);
                list.push(inner_element);
            }
            ']' => {
                break;
            }
            ',' => continue,
            _ => panic!("bad char:  {}", char),
        }
    }

    Element::List(list)
}

fn is_in_correct_order(left: &Element, right: &Element) -> Option<bool> {
    if let (Element::Value(left), Element::Value(right)) = (left, right) {
        if left == right {
            None
        } else {
            Some(left < right)
        }
    } else if let (Element::List(left), Element::List(right)) = (left, right) {
        for (a, b) in left.iter().zip(right.iter()) {
            let result = is_in_correct_order(a, b);
            if result.is_some() {
                return result;
            }
        }

        if left.len() == right.len() {
            None
        } else {
            Some(left.len() < right.len())
        }
    } else if let Element::Value(left) = left {
        let converted_left = Element::List(vec![Element::Value(*left)]);
        is_in_correct_order(&converted_left, right)
    } else if let Element::Value(right) = right {
        let converted_right = Element::List(vec![Element::Value(*right)]);
        is_in_correct_order(left, &converted_right)
    } else {
        panic!("oops");
    }
}

fn partition(packets: Vec<Element>, partition: Element) -> (Vec<Element>, Vec<Element>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for packet in packets {
        if is_in_correct_order(&packet, &partition).unwrap_or(true) {
            left.push(packet);
        } else {
            right.push(packet);
        }
    }

    right.insert(0, partition);
    (left, right)
}
