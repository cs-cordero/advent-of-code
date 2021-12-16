use advent_of_code::*;
use std::collections::VecDeque;

fn main() {
    let hex = read_input_as_string("2021/day16/src/input.txt")
        .trim()
        .to_string();

    let answer1 = sum_packet_versions(&consume_packet(&mut to_bits(&hex)));
    let answer2 = evaluate_packet(&consume_packet(&mut to_bits(&hex)));

    println!("Part 1: {:?}", answer1);
    println!("Part 2: {:?}", answer2);
}

#[derive(Copy, Clone, Debug)]
struct PacketLiteral {
    version: usize,
    value: usize,
    packet_length: usize,
}

#[derive(Clone, Debug)]
struct PacketOperator {
    version: usize,
    packet_type: usize,
    sub_packets: Vec<Packet>,
    packet_length: usize,
}

#[derive(Clone, Debug)]
enum Packet {
    Literal(PacketLiteral),
    Operator(PacketOperator),
}

impl Packet {
    fn get_packet_length(&self) -> usize {
        match self {
            Packet::Literal(data) => data.packet_length,
            Packet::Operator(data) => data.packet_length,
        }
    }
}

fn consume_packet(bit_stream: &mut VecDeque<usize>) -> Packet {
    let mut packet_size = 0;
    let packet_version = consume_bits(bit_stream, &mut packet_size, 3);
    let packet_type_id = consume_bits(bit_stream, &mut packet_size, 3);

    match packet_type_id {
        4 => {
            let mut result = 0;

            loop {
                let read_bit = consume_bits(bit_stream, &mut packet_size, 1);
                let digit = consume_bits(bit_stream, &mut packet_size, 4);
                result <<= 4;
                result |= digit;

                if read_bit == 0 {
                    break;
                }
            }

            Packet::Literal(PacketLiteral {
                version: packet_version,
                value: result,
                packet_length: packet_size,
            })
        }
        _ => {
            let length_type_id = consume_bits(bit_stream, &mut packet_size, 1);

            let mut sub_packets: Vec<Packet> = Vec::new();
            if length_type_id == 0 {
                let mut remaining_bits = consume_bits(bit_stream, &mut packet_size, 15) as i32;
                while remaining_bits > 0 {
                    let sub_packet = consume_packet(bit_stream);
                    sub_packets.push(sub_packet.clone());

                    let packet_length = match sub_packet {
                        Packet::Literal(packet_data) => packet_data.packet_length,
                        Packet::Operator(packet_data) => packet_data.packet_length,
                    };
                    packet_size += packet_length;
                    remaining_bits -= packet_length as i32;
                }
            } else {
                let sub_packet_count = consume_bits(bit_stream, &mut packet_size, 11);

                for _ in 0..sub_packet_count {
                    let sub_packet = consume_packet(bit_stream);
                    packet_size += sub_packet.get_packet_length();
                    sub_packets.push(sub_packet);
                }
            }

            Packet::Operator(PacketOperator {
                version: packet_version,
                packet_type: packet_type_id,
                sub_packets,
                packet_length: packet_size,
            })
        }
    }
}

#[inline]
fn consume_bits(bit_stream: &mut VecDeque<usize>, packet_size: &mut usize, bits: usize) -> usize {
    *packet_size += bits;
    (0..bits)
        .into_iter()
        .map(|_| bit_stream.pop_front().unwrap_or(0))
        .fold(0usize, |mut acc, bit| {
            assert!(bit == 0 || bit == 1);
            acc <<= 1;
            acc += bit;
            acc
        })
}

fn to_bits(s: &str) -> VecDeque<usize> {
    s.chars()
        .flat_map(|char| match char {
            '0' => vec![0, 0, 0, 0],
            '1' => vec![0, 0, 0, 1],
            '2' => vec![0, 0, 1, 0],
            '3' => vec![0, 0, 1, 1],
            '4' => vec![0, 1, 0, 0],
            '5' => vec![0, 1, 0, 1],
            '6' => vec![0, 1, 1, 0],
            '7' => vec![0, 1, 1, 1],
            '8' => vec![1, 0, 0, 0],
            '9' => vec![1, 0, 0, 1],
            'A' => vec![1, 0, 1, 0],
            'B' => vec![1, 0, 1, 1],
            'C' => vec![1, 1, 0, 0],
            'D' => vec![1, 1, 0, 1],
            'E' => vec![1, 1, 1, 0],
            'F' => vec![1, 1, 1, 1],
            _ => panic!("Invalid hexadecimal value"),
        })
        .collect::<VecDeque<usize>>()
}

fn sum_packet_versions(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(data) => data.version,
        Packet::Operator(data) => {
            data.version
                + data
                    .sub_packets
                    .iter()
                    .map(|sub| sum_packet_versions(sub))
                    .sum::<usize>()
        }
    }
}

fn evaluate_packet(packet: &Packet) -> usize {
    match packet {
        Packet::Literal(data) => data.value,
        Packet::Operator(data) => {
            let mut values = data
                .sub_packets
                .iter()
                .map(|sub_packet| evaluate_packet(sub_packet));
            match data.packet_type {
                0 => values.sum(),
                1 => values.product(),
                2 => values.min().unwrap(),
                3 => values.max().unwrap(),
                5 => (values.next() > values.next()) as usize,
                6 => (values.next() < values.next()) as usize,
                7 => (values.next() == values.next()) as usize,
                _ => panic!("Invalid packet type"),
            }
        }
    }
}
