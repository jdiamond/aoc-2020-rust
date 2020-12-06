#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::{self, Read};

mod test_helpers;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

const FNS: [[fn(&str) -> String; 2]; 6] = [
    [day1::part1, day1::part2],
    [day2::part1, day2::part2],
    [day3::part1, day3::part2],
    [day4::part1, day4::part2],
    [day5::part1, day5::part2],
    [day6::part1, day6::part2],
];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: cargo run <day> <part> < input.txt");
    } else {
        let day = &args[1].parse::<usize>().unwrap();
        let part = &args[2].parse::<usize>().unwrap();

        println!("running day {} part {}", day, part);

        let mut input = String::new();

        io::stdin().read_to_string(&mut input).unwrap();

        let found = FNS[day - 1][part - 1];

        let output = found(&input);

        println!("{}", output);
    }
}
