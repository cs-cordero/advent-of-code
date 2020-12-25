use advent_of_code::*;

enum Agent {
    Card,
    Door,
}

fn main() {
    let mut input = read_input_as_lines("2020/day25/src/input.txt")
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap());

    let card_public_key = input.next().unwrap();
    let door_public_key = input.next().unwrap();

    let (loop_size, agent) = find_first_loop_size(card_public_key, door_public_key);
    let encryption_key = match agent {
        Agent::Card => transform(door_public_key, loop_size),
        Agent::Door => transform(card_public_key, loop_size),
    };

    println!("Part 1: {}", encryption_key);
    println!("Part 2: Finished all other days! Merry Christmas!")
}

fn transform(subject_number: u64, loop_size: u64) -> u64 {
    let mut result = 1;
    for _ in 0..loop_size {
        result *= subject_number;
        result %= 20201227;
    }
    result
}

fn find_first_loop_size(card_public_key: u64, door_public_key: u64) -> (u64, Agent) {
    let subject_number = 7;
    let mut public_key = 1;
    let mut loop_size = 0;

    loop {
        loop_size += 1;
        public_key *= subject_number;
        public_key %= 20201227;
        if public_key == card_public_key {
            return (loop_size, Agent::Card);
        } else if public_key == door_public_key {
            return (loop_size, Agent::Door);
        }
    }
}
