use std::cmp::{min, Ordering};
use std::collections::{HashMap, VecDeque};
use std::ops::RangeInclusive;

use advent_of_code::*;

fn main() {
    let (seeds, almanacs) = {
        let raw = read_input_as_string("2023/day05/src/input.txt");
        let (seeds, raw_almanacs) = raw.split_once("\n\n").unwrap();

        let seeds: Vec<u64> = {
            let (_, seeds) = seeds.split_once(": ").unwrap();
            seeds
                .split_ascii_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        };

        let almanacs = raw_almanacs
            .split("\n\n")
            .map(|chunk| {
                let lines = chunk.lines().skip(1);
                Almanac::parse(lines)
            })
            .collect::<Vec<_>>();

        (seeds, almanacs)
    };

    let part1 = seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            for almanac in almanacs.iter() {
                value = almanac.map(value)
            }
            value
        })
        .min()
        .unwrap();

    let part2 = {
        let mut seed_ranges = seeds
            .chunks_exact(2)
            .map(|chunk| {
                let seed = chunk[0];
                let range = chunk[1];
                seed..=(seed + range - 1)
            })
            .collect::<Vec<_>>();

        for almanac in almanacs {
            seed_ranges = seed_ranges
                .iter()
                .flat_map(|range| almanac.map_range(range.clone()))
                .collect::<Vec<_>>()
        }

        seed_ranges
            .iter()
            .map(|range| *range.start())
            .min()
            .unwrap()
    };
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}

#[derive(Default)]
struct Almanac {
    source_to_range: HashMap<u64, u64>,
    source_to_destination: HashMap<u64, u64>,
    sources: Vec<u64>,
}

impl Almanac {
    fn parse<'a>(lines: impl IntoIterator<Item = &'a str>) -> Almanac {
        let mut source_to_range = HashMap::new();
        let mut source_to_destination = HashMap::new();
        let mut sources = Vec::new();

        for line in lines {
            let (destination, rest) = line.split_once(' ').unwrap();
            let (source, range) = rest.split_once(' ').unwrap();

            let destination = destination.parse::<u64>().unwrap();
            let source = source.parse::<u64>().unwrap();
            let range = range.parse::<u64>().unwrap();

            source_to_range.insert(source, range);
            source_to_destination.insert(source, destination);
            sources.push(source);
        }

        sources.sort();

        Almanac {
            source_to_range,
            source_to_destination,
            sources,
        }
    }

    fn map(&self, source: u64) -> u64 {
        if let Some(i) = self.bin(source) {
            let almanac_source = *self.sources.get(i).unwrap();
            let almanac_range = *self.source_to_range.get(&almanac_source).unwrap();
            if (almanac_source..=(almanac_source + almanac_range - 1)).contains(&source) {
                let almanac_destination = self.source_to_destination.get(&almanac_source).unwrap();
                let delta = source - almanac_source;
                almanac_destination + delta
            } else {
                source
            }
        } else {
            source
        }
    }

    fn map_range(&self, range: RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut queue = VecDeque::new();
        queue.push_back(range);

        let mut result = Vec::new();
        while let Some(range_to_map) = queue.pop_front() {
            let i = self.bin(*range_to_map.start());
            if i.is_none() {
                result.push(range_to_map);
                continue;
            }
            let i = i.unwrap();

            let source_start = self.sources.get(i).unwrap();
            let source_size = self.source_to_range.get(source_start).unwrap();
            let source_end = *source_start + source_size - 1;
            let source_range = *source_start..=source_end;

            let has_overlap = range_to_map.start() <= source_range.end()
                && range_to_map.end() >= source_range.start();
            if !has_overlap {
                result.push(range_to_map);
                continue;
            }

            let destination = *self.source_to_destination.get(source_start).unwrap();
            let delta = range_to_map.start() - source_start;
            let destination_start = destination + delta;

            let destination_range = min(*range_to_map.end(), source_end) - range_to_map.start();
            let destination_end = destination_start + destination_range;

            result.push(destination_start..=destination_end);

            let match_size = destination_end - destination_start;
            let unmatched_right = (range_to_map.start() + match_size + 1)..=*range_to_map.end();

            if !unmatched_right.is_empty() {
                queue.push_back(unmatched_right)
            }
        }
        result
    }

    fn bin(&self, needle: u64) -> Option<usize> {
        if needle < *self.sources.first().unwrap() {
            return None;
        }

        let mut left = 0;
        let mut right = self.sources.len();
        while left < right {
            let mid = (right + left) / 2;
            let value = self.sources.get(mid).unwrap();
            match needle.cmp(value) {
                Ordering::Greater => {
                    if left == mid {
                        return Some(left);
                    }
                    left = mid;
                }
                Ordering::Equal => {
                    return Some(mid);
                }
                Ordering::Less => {
                    right = mid;
                }
            }
        }

        Some(left)
    }
}
