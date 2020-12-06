use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub fn read_input_as_digits<P: AsRef<Path>>(path: P) -> Vec<u32> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|char| char.to_digit(10).unwrap_or_else(|| panic!("Failed to parse {}", char)))
        .collect()
}

pub fn read_input_as_string<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .to_owned()
}

pub fn read_input_as_string_vec<P: AsRef<Path>>(path: P) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|char| char.to_string())
        .collect()
}

pub fn read_input_as_lines<P: AsRef<Path>>(path: P) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

pub fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut result = HashMap::new();
    s.chars()
        .for_each(|c| {
            let value = result.get(&c).unwrap_or(&0) + 1;
            result.insert(c, value);
        });
    result
}

pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}
