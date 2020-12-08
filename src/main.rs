#[macro_use]
extern crate lazy_static;

use clap::{App, Arg};
use std::io::{self, Read};

mod test_helpers;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

const FNS: [[fn(&str) -> String; 2]; 8] = [
    [day1::part1, day1::part2],
    [day2::part1, day2::part2],
    [day3::part1, day3::part2],
    [day4::part1, day4::part2],
    [day5::part1, day5::part2],
    [day6::part1, day6::part2],
    [day7::part1, day7::part2],
    [day8::part1, day8::part2],
];

fn main() {
    let matches = App::new("aoc-2020-rust")
        .usage("cargo run -- --day DAY --part PART < inputs/INPUT.txt")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .value_name("DAY")
                .help("Day number")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .help("Part number")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap().parse::<usize>().unwrap();
    let part = matches.value_of("part").unwrap().parse::<usize>().unwrap();

    println!("running day {} part {}", day, part);

    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let found = FNS[day - 1][part - 1];

    let output = found(&input);

    println!("{}", output);
}
