use advent_of_code::*;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

fn main() {
    let input = read_input_as_string("2020/day04/src/input.txt");
    let passports = process_passports(input);

    let required_fields = {
        let mut req = HashSet::new();
        req.insert(PassportField::BirthYear);
        req.insert(PassportField::IssueYear);
        req.insert(PassportField::ExpirationYear);
        req.insert(PassportField::Height);
        req.insert(PassportField::HairColor);
        req.insert(PassportField::EyeColor);
        req.insert(PassportField::PassportID);
        req
    };

    let answer1 = passports
        .iter()
        .filter(|passport| {
            required_fields
                .iter()
                .all(|required| passport.contains_key(required))
        })
        .count();

    let answer2 = passports
        .iter()
        .filter(|passport| {
            required_fields
                .iter()
                .all(|required| {
                    passport
                        .get(required)
                        .and_then(|value| Some(required.is_valid(value)))
                        .unwrap_or(false)
                })
        })
        .count();

    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn process_passports(input: String) -> Vec<HashMap<PassportField, String>> {
    input
        .split("\n\n")
        .map(|chunk|
            chunk
                .lines()
                .flat_map(|line| line.split(" "))
                .map(|data| {
                    let mut raw = data.split(":");
                    let key = raw.next().unwrap();
                    let value = raw.next().unwrap().to_owned();
                    (PassportField::from_str(key).unwrap(), value)
                })
                .collect::<HashMap<_, _>>()
        )
        .collect::<Vec<_>>()
}

#[derive(Eq, Hash, PartialEq)]
pub enum PassportField {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
}

impl FromStr for PassportField {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "byr" => Ok(Self::BirthYear),
            "iyr" => Ok(Self::IssueYear),
            "eyr" => Ok(Self::ExpirationYear),
            "hgt" => Ok(Self::Height),
            "hcl" => Ok(Self::HairColor),
            "ecl" => Ok(Self::EyeColor),
            "pid" => Ok(Self::PassportID),
            "cid" => Ok(Self::CountryID),
            _ => Err(format!("Bad Passport Field {}", s))
        }
    }
}

impl PassportField {
    pub fn is_valid(&self, value: &str) -> bool {
        match self {
            Self::BirthYear => value.parse::<i32>().ok().map(|year| 1920 <= year && year <= 2002).unwrap_or(false),
            Self::IssueYear => value.parse::<i32>().ok().map(|year| 2010 <= year && year <= 2020).unwrap_or(false),
            Self::ExpirationYear => value.parse::<i32>().ok().map(|year| 2020 <= year && year <= 2030).unwrap_or(false),
            Self::Height => {
                let len = value.len();
                value[..len-2].parse::<i32>().ok().map(|height|
                    match &value[len-2..] {
                        "cm" => 150 <= height && height <= 193,
                        "in" => 59 <= height && height <= 76,
                        _ => false
                    }
                ).unwrap_or(false)
            },
            Self::HairColor => {
                &value[..1] == "#" && value[1..].chars().all(|c| c.is_ascii_hexdigit())
            },
            Self::EyeColor => {
                match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false
                }
            },
            Self::PassportID => value.len() == 9 && value.parse::<i32>().is_ok(),
            Self::CountryID => true,
        }
    }
}
