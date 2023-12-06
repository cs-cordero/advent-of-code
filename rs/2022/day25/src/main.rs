extern crate core;

use advent_of_code::*;

fn main() {
    let sum_decimal = read_input_as_lines("2022/day25/src/input.txt")
        .into_iter()
        .map(|line| from_snafu(&line))
        .sum::<i64>();

    println!("Part 1: {}", to_snafu(sum_decimal));
    println!("Part 2: Click the button");
}

fn from_snafu(snafu: &str) -> i64 {
    let mut decimal = 0;
    for (place, raw) in snafu.chars().rev().enumerate() {
        let value: i64 = match raw {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => panic!("Invalid SNAFU number."),
        };

        decimal += value * u64::pow(5, place as u32) as i64;
    }

    decimal
}

fn to_snafu(decimal: i64) -> String {
    let mut larger_snafu_num = String::from("2");
    while from_snafu(&larger_snafu_num) < decimal {
        larger_snafu_num.push('2');
    }

    to_snafu_helper(decimal, larger_snafu_num.len())
}

fn to_snafu_helper(decimal: i64, digit_count: usize) -> String {
    if digit_count == 0 {
        return String::from("");
    }

    let sign: i64 = decimal.signum();
    let target = i64::abs(decimal);

    let larger_range = {
        let mut s = String::from("2");
        let mut t = String::from("2");
        while s.len() < digit_count {
            s.push('2');
            t.push('=');
        }
        from_snafu(&t)..=from_snafu(&s)
    };
    let smaller_range = {
        let mut s = String::from("1");
        let mut t = String::from("1");
        while s.len() < digit_count {
            s.push('2');
            t.push('=');
        }
        from_snafu(&t)..=from_snafu(&s)
    };

    let in_larger_range = larger_range.contains(&target);
    let in_smaller_range = smaller_range.contains(&target);

    let place = u64::pow(5, digit_count as u32 - 1) as i64;
    let nominal_value = if in_larger_range && !in_smaller_range {
        // in larger range, the digit should be 2 or =.
        2
    } else if !in_larger_range && in_smaller_range {
        // in smaller range, the digit should be 1 or -.
        1
    } else {
        // in neither range, the digit should be 0.
        0
    };
    let next_decimal_value = decimal - (place * nominal_value * sign);

    let mut result = String::from(match nominal_value * sign {
        -2 => "=",
        -1 => "-",
        0 => "0",
        1 => "1",
        2 => "2",
        _ => panic!("Not possible"),
    });
    result.push_str(&to_snafu_helper(next_decimal_value, digit_count - 1));
    result
}
