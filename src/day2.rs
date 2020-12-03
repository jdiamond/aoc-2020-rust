use regex::Regex;
use std::io::{self, BufRead};

pub fn part1() {
    run(parse_line1);
}

pub fn part2() {
    run(parse_line2);
}

type LineParser = fn(line: &str) -> (Box<dyn PasswordPolicy>, String);

fn run(parse_line: LineParser) {
    let stdin = io::stdin();

    let records = stdin.lock().lines().map(|line| parse_line(&line.unwrap()));

    let mut total = 0;
    let mut valid = 0;

    for record in records {
        total += 1;

        let is_valid = record.0.validate(&record.1);

        if is_valid {
            valid += 1;
        }

        // println!("{:?}: {}", record, is_valid);
    }

    println!("there are {} valid passwords out of {} total", valid, total,)
}

trait PasswordPolicy {
    fn validate(&self, password: &str) -> bool;
}

#[derive(Debug)]
struct PasswordPolicy1 {
    lowest: usize,
    highest: usize,
    letter: char,
}

impl PasswordPolicy for PasswordPolicy1 {
    fn validate(&self, password: &str) -> bool {
        let count = password.chars().fold(0, |count, char| {
            if char == self.letter {
                count + 1
            } else {
                count
            }
        });

        count >= self.lowest && count <= self.highest
    }
}

fn parse_line1(line: &str) -> (Box<dyn PasswordPolicy>, String) {
    // println!("{}", line);

    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(line).unwrap();

    // println!("{:?}", caps);

    (
        Box::new(PasswordPolicy1 {
            lowest: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            highest: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        }),
        caps.get(4).unwrap().as_str().to_string(),
    )
}

#[derive(Debug)]
struct PasswordPolicy2 {
    first: usize,
    second: usize,
    letter: char,
}

impl PasswordPolicy for PasswordPolicy2 {
    fn validate(&self, password: &str) -> bool {
        let first = password.chars().nth(self.first - 1).unwrap();
        let second = password.chars().nth(self.second - 1).unwrap();

        (first == self.letter && second != self.letter)
            || (first != self.letter && second == self.letter)
    }
}

fn parse_line2(line: &str) -> (Box<dyn PasswordPolicy>, String) {
    // println!("{}", line);

    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(line).unwrap();

    // println!("{:?}", caps);

    (
        Box::new(PasswordPolicy2 {
            first: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            second: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        }),
        caps.get(4).unwrap().as_str().to_string(),
    )
}
