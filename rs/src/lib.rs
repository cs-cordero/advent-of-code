use std::fs;
use std::path::Path;

pub fn read_input_as_digits<P: AsRef<Path>>(path: P) -> Vec<u32> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|char| char.to_digit(10).expect(&format!("Failed to parse {}", char)))
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
        .split("\n")
        .map(|line| line.to_owned())
        .collect()
}
