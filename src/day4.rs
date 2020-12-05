use regex::Regex;
use std::collections::HashMap;

pub fn part1(input: &str) -> String {
    let passports = parse(input);

    // dbg!(passports);

    let valid = passports
        .into_iter()
        .filter(|passport| is_valid(passport))
        .count();

    valid.to_string()
}

pub fn part2(_input: &str) -> String {
    todo!()
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

fn is_valid(passport: &Passport) -> bool {
    // dbg!(passport);

    REQUIRED_FIELDS
        .iter()
        .all(|key| passport.fields.contains_key(*key))
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

    // #[test]
    // fn part2_example_works() {
    //     let input = load_input("day4-example.txt");

    //     assert_eq!(part2(&input), "");
    // }

    // #[test]
    // fn part2_input_works() {
    //     let input = load_input("day4-puzzle.txt");

    //     assert_eq!(part2(&input), "");
    // }
}
