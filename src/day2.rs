use std::io::{self, BufRead};

pub fn part1() {
    let stdin = io::stdin();

    let records = stdin.lock().lines().map(|line| parse_line(&line.unwrap()));

    for record in records {
        println!("{:?}: {}", record, record.0.validate(record.1));
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

fn parse_line<'a>(_line: &str) -> (PasswordPolicy, &'a str) {
    (
        PasswordPolicy {
            lowest: 0,
            highest: 0,
            letter: 'a',
        },
        "abc",
    )
}

pub fn part2() {}
