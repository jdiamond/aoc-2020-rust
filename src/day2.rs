use regex::Regex;
use std::io::{self, BufRead};

pub fn part1() {
    run(validate1);
}

pub fn part2() {
    run(validate2);
}

type Validator = fn(policy: &PasswordPolicy, password: &str) -> bool;

fn run(validate: Validator) {
    let stdin = io::stdin();

    let records = stdin.lock().lines().map(|line| parse_line(&line.unwrap()));

    let mut total = 0;
    let mut valid = 0;

    for record in records {
        total += 1;

        let is_valid = validate(&record.0, &record.1);

        if is_valid {
            valid += 1;
        }

        // println!("{:?}: {}", record, is_valid);
    }

    println!("there are {} valid passwords out of {} total", valid, total,)
}

#[derive(Debug)]
struct PasswordPolicy {
    first: usize,
    second: usize,
    letter: char,
}

fn parse_line(line: &str) -> (PasswordPolicy, String) {
    // println!("{}", line);

    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(line).unwrap();

    // println!("{:?}", caps);

    (
        PasswordPolicy {
            first: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            second: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        },
        caps.get(4).unwrap().as_str().to_string(),
    )
}

fn validate1(policy: &PasswordPolicy, password: &str) -> bool {
    let count = password.chars().fold(0, |count, char| {
        if char == policy.letter {
            count + 1
        } else {
            count
        }
    });

    count >= policy.first && count <= policy.second
}

fn validate2(policy: &PasswordPolicy, password: &str) -> bool {
    let first = password.chars().nth(policy.first - 1).unwrap();
    let second = password.chars().nth(policy.second - 1).unwrap();

    (first == policy.letter && second != policy.letter)
        || (first != policy.letter && second == policy.letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate1_works() {
        assert!(validate1(
            &PasswordPolicy {
                first: 1,
                second: 3,
                letter: 'a',
            },
            "abcde",
        ));
    }

    #[test]
    fn validate2_works() {
        assert!(validate2(
            &PasswordPolicy {
                first: 1,
                second: 3,
                letter: 'a',
            },
            "abcde",
        ));
    }
}
