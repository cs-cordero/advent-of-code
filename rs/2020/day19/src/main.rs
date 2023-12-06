use advent_of_code::*;
use std::cmp::min;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input_as_string("2020/day19/src/input.txt");
    let mut input_split = input.split("\n\n");

    let raw_rules = input_split.next().unwrap();
    let messages = input_split.next().unwrap();

    let rules = parse_rules(raw_rules);
    let answer1 = {
        let rule = rules.get(&0).unwrap();
        messages.lines().filter(|&s| rule.contains(s)).count()
    };

    let answer2 = {
        let rule_42 = rules.get(&42).unwrap();
        let rule_31 = rules.get(&31).unwrap();
        messages
            .lines()
            .filter(|&s| aggressively_match_8_and_11(s, rule_42, rule_31))
            .count()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn parse_rules(s: &str) -> HashMap<usize, HashSet<String>> {
    let mut result = HashMap::new();

    let mut temp = HashMap::new();
    for line in s.lines() {
        let (rule_id, raw_rule) = line.split_once(": ").unwrap();
        let rule_id = rule_id.parse::<usize>().unwrap();
        match raw_rule {
            "\"a\"" | "\"b\"" => {
                let mut inner_result = HashSet::new();
                inner_result.insert(raw_rule.replace('"', "").to_owned());
                result.insert(rule_id, inner_result);
            }
            _ => {
                temp.insert(
                    rule_id,
                    raw_rule
                        .split(" | ")
                        .map(|sub_rule| sub_rule.split(' ').collect::<Vec<_>>())
                        .collect::<Vec<_>>(),
                );
            }
        }
    }

    recursive_rule_parsing(result, &mut HashSet::new(), &temp, 0)
}

fn recursive_rule_parsing<'a>(
    mut result: HashMap<usize, HashSet<String>>,
    visiting: &'a mut HashSet<usize>,
    raw_rules: &'a HashMap<usize, Vec<Vec<&'a str>>>,
    current: usize,
) -> HashMap<usize, HashSet<String>> {
    if result.contains_key(&current) || visiting.contains(&current) {
        return result;
    }

    let mut inner_result = HashSet::new();

    let raw = &raw_rules[&current];
    for sub_rules in raw {
        let indexes = sub_rules
            .iter()
            .map(|i| i.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for i in indexes.iter() {
            if !visiting.contains(i) {
                result = recursive_rule_parsing(result, visiting, raw_rules, *i);
                visiting.insert(*i);
            }
        }

        inner_result.extend(combine("".to_owned(), 0, &indexes, &result));
    }

    result.insert(current, inner_result);
    result
}

fn combine(
    s: String,
    i: usize,
    ordered_rules: &[usize],
    raw_rules: &HashMap<usize, HashSet<String>>,
) -> HashSet<String> {
    let mut result = HashSet::new();

    if i == ordered_rules.len() {
        result.insert(s);
        return result;
    }

    let possible_rules_at_i = raw_rules.get(&ordered_rules[i]).unwrap();
    for possible_rule in possible_rules_at_i {
        let mut s = s.clone();
        s.push_str(possible_rule);
        result.extend(combine(s, i + 1, ordered_rules, raw_rules));
    }

    result
}

fn aggressively_match_8_and_11(
    s: &str,
    rule_42: &HashSet<String>,
    rule_31: &HashSet<String>,
) -> bool {
    let rule_len = rule_42.iter().next().unwrap().len();
    let count_31_from_right = (1..=s.len())
        .rev()
        .step_by(rule_len)
        .map(|i| &s[i..min(i + rule_len, s.len())])
        .skip(1)
        .take_while(|&s| rule_31.contains(s))
        .count();
    let count_42_from_left = (0..s.len())
        .step_by(rule_len)
        .map(|i| &s[i..min(i + rule_len, s.len())])
        .take_while(|&s| rule_42.contains(s))
        .count();

    let expected_count = s.len() / rule_len;
    count_31_from_right + count_42_from_left == expected_count
        && count_42_from_left > count_31_from_right
        && count_31_from_right > 0
        && count_42_from_left > 0
}
