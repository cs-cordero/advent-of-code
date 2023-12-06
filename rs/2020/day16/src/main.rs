use advent_of_code::*;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Rule {
    DepartureLocation,
    DepartureStation,
    DeparturePlatform,
    DepartureTrack,
    DepartureDate,
    DepartureTime,
    ArrivalLocation,
    ArrivalStation,
    ArrivalPlatform,
    ArrivalTrack,
    Class,
    Duration,
    Price,
    Route,
    Row,
    Seat,
    Train,
    Type,
    Wagon,
    Zone,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "departure location" => Ok(Rule::DepartureLocation),
            "departure station" => Ok(Rule::DepartureStation),
            "departure platform" => Ok(Rule::DeparturePlatform),
            "departure track" => Ok(Rule::DepartureTrack),
            "departure date" => Ok(Rule::DepartureDate),
            "departure time" => Ok(Rule::DepartureTime),
            "arrival location" => Ok(Rule::ArrivalLocation),
            "arrival station" => Ok(Rule::ArrivalStation),
            "arrival platform" => Ok(Rule::ArrivalPlatform),
            "arrival track" => Ok(Rule::ArrivalTrack),
            "class" => Ok(Rule::Class),
            "duration" => Ok(Rule::Duration),
            "price" => Ok(Rule::Price),
            "route" => Ok(Rule::Route),
            "row" => Ok(Rule::Row),
            "seat" => Ok(Rule::Seat),
            "train" => Ok(Rule::Train),
            "type" => Ok(Rule::Type),
            "wagon" => Ok(Rule::Wagon),
            "zone" => Ok(Rule::Zone),
            _ => Err("Invalid!".to_string()),
        }
    }
}

fn main() {
    let mut input_lines = read_input_as_string("2020/day16/src/input.txt")
        .split("\n\n")
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
        .into_iter();

    let rules = input_lines
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (rule_name, ranges) = line.split_once(": ").unwrap();
            let parsed_ranges = ranges
                .split(" or ")
                .map(|raw_split| {
                    let (low, high) = raw_split.split_once('-').unwrap();
                    low.parse::<u32>().unwrap()..=high.parse::<u32>().unwrap()
                })
                .collect::<Vec<_>>();
            (Rule::from_str(rule_name).unwrap(), parsed_ranges)
        })
        .collect::<HashMap<Rule, Vec<RangeInclusive<u32>>>>();

    let my_ticket = input_lines
        .next()
        .unwrap()
        .lines()
        .nth(1)
        .unwrap()
        .to_owned();

    let other_tickets = input_lines
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|value| value.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let answer1 = {
        other_tickets
            .iter()
            .flat_map(|ticket| {
                ticket
                    .iter()
                    .filter(|value| !matches_at_least_one_rule(&rules, **value))
            })
            .sum::<u32>()
    };

    let answer2 = {
        let target_rules = {
            let mut result = HashSet::new();
            result.insert(Rule::DepartureLocation);
            result.insert(Rule::DepartureStation);
            result.insert(Rule::DeparturePlatform);
            result.insert(Rule::DepartureTrack);
            result.insert(Rule::DepartureDate);
            result.insert(Rule::DepartureTime);
            result
        };

        let mut index_to_possible_rules = {
            // First, create a mapping of indexes to Vec<HashSet<Rule>>
            let mut index_to_all_ticket_possibilities = HashMap::new();
            other_tickets
                .iter()
                .filter(|ticket| {
                    ticket
                        .iter()
                        .all(|value| matches_at_least_one_rule(&rules, *value))
                })
                .for_each(|ticket| {
                    ticket.iter().enumerate().for_each(|(i, value)| {
                        let possible_rules = rules
                            .iter()
                            .filter(|(_, ranges)| ranges.iter().any(|range| range.contains(value)))
                            .map(|(rule, _)| *rule)
                            .collect::<HashSet<_>>();

                        index_to_all_ticket_possibilities
                            .entry(i)
                            .or_insert_with(Vec::new)
                            .push(possible_rules);
                    })
                });

            // Second, create the final map by intersecting all HashSets for each index
            let mut result = HashMap::new();
            for (index, possibilities) in index_to_all_ticket_possibilities {
                let mut sets = possibilities.into_iter();
                result.insert(
                    index,
                    sets.next().map(|v| sets.fold(v, |a, b| &a & &b)).unwrap(),
                );
            }
            result
        };

        // Solver
        let mut index_to_final_rule = HashMap::new();
        while !index_to_possible_rules.is_empty() {
            if let Some((index, rule)) = deduce_required_index_rule_pair(&index_to_possible_rules) {
                index_to_final_rule.insert(index, rule);
                prune(&mut index_to_possible_rules, rule);
            } else {
                panic!("Unable to deduce a rule to remove!");
            }
        }

        my_ticket
            .split(',')
            .map(|value| value.parse::<u32>().unwrap())
            .enumerate()
            .filter_map(|(i, value)| {
                index_to_final_rule
                    .get(&i)
                    .filter(|rule| target_rules.contains(rule))
                    .map(|_| value as u64)
            })
            .product::<u64>()
    };

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

/// Locates an index where the possible rules has one and exactly one available
#[inline]
fn deduce_required_index_rule_pair(
    possibilities: &HashMap<usize, HashSet<Rule>>,
) -> Option<(usize, Rule)> {
    for (index, possible_rules) in possibilities {
        if possible_rules.len() == 1 {
            return Some((*index, *possible_rules.iter().next().unwrap()));
        }
    }
    None
}

/// Removes a given rule from all the sets in the map, then removes any keys with empty sets.
#[inline]
fn prune(map: &mut HashMap<usize, HashSet<Rule>>, rule_to_remove: Rule) {
    for (_, possible_rules) in map.iter_mut() {
        possible_rules.remove(&rule_to_remove);
    }
    map.retain(|_, possible_rules| !possible_rules.is_empty());
}

#[inline]
fn matches_at_least_one_rule(
    rule_range_map: &HashMap<Rule, Vec<RangeInclusive<u32>>>,
    value: u32,
) -> bool {
    rule_range_map
        .values()
        .any(|rule_ranges| rule_ranges.iter().any(|range| range.contains(&value)))
}
