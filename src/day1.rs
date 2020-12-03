use itertools::Itertools;
use std::io::{self, BufRead};
use std::str::FromStr;

pub fn part1() {
    run(2);
}

pub fn part2() {
    run(3);
}

fn run(k: usize) {
    let stdin = io::stdin();

    let results = stdin
        .lock()
        .lines()
        .map(|line| u32::from_str(&line.unwrap()).unwrap())
        .combinations(k)
        .filter(|combo| combo.iter().sum::<u32>() == 2020);

    for result in results {
        println!("{:?}: {}", result, result.iter().product::<u32>());
    }
}
