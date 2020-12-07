use advent_of_code::*;

fn main() {
    let parsed_lines: Vec<(u32, u32, char, String)> =
        read_input_as_lines("2020/day02/src/input.txt")
            .into_iter()
            .map(parse_line)
            .collect();

    let answer1 = parsed_lines
        .iter()
        .filter(|(low, high, letter, password)| {
            let freq = count_chars(password);
            let letter_freq = freq.get(letter).unwrap_or(&0);
            low <= letter_freq && letter_freq <= high
        })
        .count();

    let answer2 = parsed_lines
        .iter()
        .filter(|(low, high, letter, password)| {
            let match1 = &password.chars().nth((low - 1) as usize).unwrap() == letter;
            let match2 = &password.chars().nth((high - 1) as usize).unwrap() == letter;
            (match1 || match2) && (match1 != match2)
        })
        .count();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn parse_line(line: String) -> (u32, u32, char, String) {
    let mut split = line.split(' ');
    let (low, high) = {
        let range: Vec<u32> = split
            .next()
            .unwrap()
            .split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        (range[0], range[1])
    };
    let letter = first_char(split.next().unwrap()).unwrap();
    let password = split.next().unwrap().to_owned();
    (low, high, letter, password)
}
