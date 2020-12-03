use regex::Regex;
use std::io::{self, BufRead};

pub fn part1() {
    let stdin = io::stdin();

    let records = stdin.lock().lines().map(|line| parse_line(&line.unwrap()));

    for record in records {
        println!("{:?}: {}", record, record.0.validate(&record.1));
    }
}

#[derive(Debug)]
struct PasswordPolicy {
    lowest: usize,
    highest: usize,
    letter: char,
}

impl PasswordPolicy {
    fn validate(&self, _password: &str) -> bool {
        false
    }
}

fn parse_line(line: &str) -> (PasswordPolicy, String) {
    // println!("{}", line);

    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(line).unwrap();

    // println!("{:?}", caps);

    (
        PasswordPolicy {
            lowest: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            highest: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        },
        caps.get(4).unwrap().as_str().to_string(),
    )
}

pub fn part2() {}
