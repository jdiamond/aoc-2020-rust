use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let passports = parse(input);

    // dbg!(passports);

    let valid = passports
        .into_iter()
        .filter(|passport| is_valid(passport, false))
        .count();

    valid.to_string()
}

pub fn part2(input: &str) -> String {
    let passports = parse(input);

    // dbg!(passports);

    let valid = passports
        .into_iter()
        .filter(|passport| is_valid(passport, true))
        .count();

    valid.to_string()
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

fn parse(input: &str) -> Vec<Passport> {
    let re = Regex::new(r"(\w+):(\S+)").unwrap();

    input
        .split("\n\n")
        .map(|lines| parse_passport(&re, lines))
        .collect()
}

fn parse_passport(re: &Regex, lines: &str) -> Passport {
    let fields = re
        .captures_iter(lines)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect::<HashMap<String, String>>();

    Passport { fields }
}

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn is_valid(passport: &Passport, validate_values: bool) -> bool {
    // dbg!(passport);

    REQUIRED_FIELDS.iter().all(|key| {
        passport.fields.contains_key(*key)
            && (!validate_values || is_field_valid(*key, passport.fields.get(*key).unwrap()))
    })
}

fn is_field_valid(key: &str, val: &str) -> bool {
    lazy_static! {
        static ref FOUR_DIGITS: Regex = Regex::new(r"^\d{4}$").unwrap();
        static ref HEIGHT: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        static ref HAIR_COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        static ref EYE_COLOR: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        static ref PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    match key {
        "byr" => {
            FOUR_DIGITS.is_match(val) && {
                let byr = val.parse::<usize>().unwrap();
                byr >= 1920 && byr <= 2002
            }
        }
        "iyr" => {
            FOUR_DIGITS.is_match(val) && {
                let byr = val.parse::<usize>().unwrap();
                byr >= 2010 && byr <= 2020
            }
        }
        "eyr" => {
            FOUR_DIGITS.is_match(val) && {
                let byr = val.parse::<usize>().unwrap();
                byr >= 2020 && byr <= 2030
            }
        }
        "hgt" => {
            if let Some(cap) = HEIGHT.captures(val) {
                let val = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let unit = cap.get(2).unwrap().as_str();

                match unit {
                    "cm" => val >= 150 && val <= 193,
                    "in" => val >= 59 && val <= 76,
                    _ => false,
                }
            } else {
                false
            }
        }
        "hcl" => HAIR_COLOR.is_match(val),
        "ecl" => EYE_COLOR.is_match(val),
        "pid" => PASSPORT_ID.is_match(val),
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day4-example.txt");

        assert_eq!(part1(&input), "2");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day4-puzzle.txt");

        assert_eq!(part1(&input), "170");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day4-example.txt");

        assert_eq!(part2(&input), "2");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day4-puzzle.txt");

        assert_eq!(part2(&input), "103");
    }

    #[test]
    fn field_validation() {
        assert!(is_field_valid("byr", "2002"));
        assert!(!is_field_valid("byr", "2003"));

        assert!(is_field_valid("hgt", "60in"));
        assert!(is_field_valid("hgt", "190cm"));
        assert!(!is_field_valid("hgt", "190in"));
        assert!(!is_field_valid("hgt", "190"));

        assert!(is_field_valid("hcl", "#123abc"));
        assert!(!is_field_valid("hcl", "#123abz"));
        assert!(!is_field_valid("hcl", "123abc"));

        assert!(is_field_valid("ecl", "brn"));
        assert!(!is_field_valid("ecl", "wat"));

        assert!(is_field_valid("pid", "000000001"));
        assert!(!is_field_valid("pid", "0123456789"));
    }
}
