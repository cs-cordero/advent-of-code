use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// All content in a file are read into an owned String.
///
/// Example:
///     Given:  "597348"
///     Yields: String::from("597348")
pub fn read_input_as_string<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).unwrap().trim().to_owned()
}

/// Converts numbers in a file into a vector of individual digits
/// All values in the file must be a number with radix 10.
///
/// Example:
///     Given:  "597348"
///     Yields: vec![5, 9, 7, 3, 4, 8]
pub fn read_input_as_digits<P: AsRef<Path>>(path: P) -> Vec<u32> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|char| {
            char.to_digit(10)
                .unwrap_or_else(|| panic!("Failed to parse {}", char))
        })
        .collect()
}

/// Reads all values in a file as individual, 1-character length Strings.
///
/// Example:
///     Given:  "a10"
///     Yields: vec!['a'.to_string(), '1'.to_string(), '0'.to_string()]
pub fn read_input_as_string_vec<P: AsRef<Path>>(path: P) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .chars()
        .map(|char| char.to_string())
        .collect()
}

/// Reads lines from a file into a collection of Strings.
///
/// Example:
///     Given:
///         aaa
///         bbb
///         ccc
///     Yields: vec!['aaa'.to_string(), 'bbb'.to_string(), 'ccc'.to_string()]
pub fn read_input_as_lines<P: AsRef<Path>>(path: P) -> Vec<String> {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .lines()
        .map(|line| line.to_owned())
        .collect()
}

/// Counts all chars in a slice, resulting in a hashmap of chars to their frequency.
///
/// Example:
///     Given:  "abcabaaa"
///     Yields: { "a": 5, "b": 2, "c": 1 }
pub fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut result = HashMap::new();
    s.chars().for_each(|c| {
        let value = result.get(&c).unwrap_or(&0) + 1;
        result.insert(c, value);
    });
    result
}

/// Helper method that gets the first character out of a string slice.
///
/// Example:
///     Given:  "vjbhasdfkel1"
///     Yields: Some('v')
///
/// Example:
///     Given:  ""
///     Yields: None
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}
