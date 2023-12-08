use std::cmp::Ordering;
use std::collections::HashMap;

use advent_of_code::*;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn from_char(s: &char) -> Card {
        match s {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::_9,
            '8' => Card::_8,
            '7' => Card::_7,
            '6' => Card::_6,
            '5' => Card::_5,
            '4' => Card::_4,
            '3' => Card::_3,
            '2' => Card::_2,
            _ => panic!("Invalid card"),
        }
    }

    fn default_value(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::_9 => 9,
            Card::_8 => 8,
            Card::_7 => 7,
            Card::_6 => 6,
            Card::_5 => 5,
            Card::_4 => 4,
            Card::_3 => 3,
            Card::_2 => 2,
        }
    }

    fn value_with_joker(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 1,
            Card::T => 10,
            Card::_9 => 9,
            Card::_8 => 8,
            Card::_7 => 7,
            Card::_6 => 6,
            Card::_5 => 5,
            Card::_4 => 4,
            Card::_3 => 3,
            Card::_2 => 2,
        }
    }

    fn cmp_default(&self, other: &Self) -> Ordering {
        self.default_value().cmp(&other.default_value())
    }

    fn cmp_joker(&self, other: &Self) -> Ordering {
        self.value_with_joker().cmp(&other.value_with_joker())
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Copy, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn from_s(s: &str) -> Hand {
        if !s.len() == 5 {
            panic!("Invalid hand");
        }

        let mut chars = s.chars().map(|c| Card::from_char(&c));
        let cards: [Card; 5] = [
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
            chars.next().unwrap(),
        ];

        Hand { cards }
    }

    fn hand_type(&self) -> HandType {
        let mut card_counts = HashMap::new();
        for card in self.cards.iter() {
            *card_counts.entry(*card).or_insert(0) += 1;
        }

        determine_hand_type(&card_counts)
    }

    fn hand_type_with_joker(&self) -> HandType {
        let mut joker_count = 0;
        let mut card_counts = HashMap::new();
        for card in self.cards.iter() {
            if card == &Card::J {
                joker_count += 1;
            } else {
                *card_counts.entry(*card).or_insert(0) += 1;
            }
        }

        if let Some((&highest_count_card, _)) = card_counts.iter().max_by_key(|(_, count)| **count)
        {
            if joker_count > 0 {
                *(card_counts.get_mut(&highest_count_card).unwrap()) += joker_count;
            }
        } else {
            card_counts.insert(Card::J, 5);
        }

        determine_hand_type(&card_counts)
    }

    fn cmp_default(&self, other: &Self) -> Ordering {
        let hand_type_order = self.hand_type().cmp(&other.hand_type());
        match hand_type_order {
            Ordering::Greater | Ordering::Less => hand_type_order,
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let card_order = self_card.cmp_default(other_card);
                    if card_order != Ordering::Equal {
                        return card_order;
                    }
                }
                Ordering::Equal
            }
        }
    }

    fn cmp_with_joker(&self, other: &Self) -> Ordering {
        let hand_type_order = self
            .hand_type_with_joker()
            .cmp(&other.hand_type_with_joker());
        match hand_type_order {
            Ordering::Greater | Ordering::Less => hand_type_order,
            Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    let card_order = self_card.cmp_joker(other_card);
                    if card_order != Ordering::Equal {
                        return card_order;
                    }
                }
                Ordering::Equal
            }
        }
    }
}

fn determine_hand_type(card_counts: &HashMap<Card, u8>) -> HandType {
    let counts = card_counts.values().copied().collect::<Vec<_>>();

    if counts.contains(&5) {
        HandType::FiveKind
    } else if counts.contains(&4) {
        HandType::FourKind
    } else if counts.contains(&3) && counts.contains(&2) {
        HandType::FullHouse
    } else if counts.contains(&3) {
        HandType::ThreeKind
    } else if counts.iter().filter(|it| **it == 2).count() == 2 {
        HandType::TwoPair
    } else if counts.contains(&2) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

fn main() {
    let mut data = read_input_as_lines("2023/day07/src/input.txt")
        .into_iter()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let hand = Hand::from_s(hand);
            let bid = bid.parse::<u32>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();
    data.sort_by(|(left, _), (right, _)| left.cmp_default(right));

    let part1: u32 = {
        data.iter()
            .enumerate()
            .map(|(i, (_, bid))| *bid * (i as u32 + 1))
            .sum()
    };

    let part2: u32 = {
        data.sort_by(|(left, _), (right, _)| left.cmp_with_joker(right));

        // data_with_joker_rule.iter().for_each(|(hand, _)| println!("{} {:?}", hand.repr(), hand.hand_type));

        data.iter()
            .enumerate()
            .map(|(i, (_, bid))| *bid * (i as u32 + 1))
            .sum()
    };

    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}
