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

/// Helper method that gets the 8 squares surrounding a given point in a 2D grid.
/// Note that the point is (row, col), where (0, 0) is in the top-left corner.
///
/// Example:
///     Given: point: (2, 2) limits_non_inclusive: (5, 5)
///     Yields: vec![(1, 1), (1, 2), (1, 3), (2, 1), (2, 3), (3, 1), (3, 2), (3, 3)]
///
/// Example:
///     Given: point: (2, 0) limits_non_inclusive: (3, 5)
///     Yields: vec![(1, 0), (1, 1), (2, 1)]
pub fn get_adjacent_points(
    point: (usize, usize),
    limits_non_inclusive: (usize, usize),
) -> Vec<(usize, usize)> {
    let (row, col) = point;
    let (row_limit, col_limit) = limits_non_inclusive;
    let prev_row = row.checked_sub(1).filter(|value| *value < row_limit);
    let next_row = row.checked_add(1).filter(|value| *value < row_limit);
    let prev_col = col.checked_sub(1).filter(|value| *value < col_limit);
    let next_col = col.checked_add(1).filter(|value| *value < col_limit);
    let row = Some(row);
    let col = Some(col);

    let mut result = Vec::new();
    for r in [prev_row, row, next_row] {
        if r.is_none() {
            continue;
        }
        for c in [prev_col, col, next_col] {
            if r == row && c == col {
                continue;
            }
            if c.is_none() {
                continue;
            }

            result.push((r.unwrap(), c.unwrap()));
        }
    }
    result
}
