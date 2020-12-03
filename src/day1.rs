use itertools::Itertools;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::vec::Vec;

pub fn part1() {
    run(2);
}

pub fn part2() {
    run(3);
}

fn run(k: usize) {
    let stdin = io::stdin();
    let mut nums = Vec::new();

    for line in stdin.lock().lines() {
        let num = u32::from_str(&line.unwrap()).unwrap();
        nums.push(num);
    }

    println!("got {} nums", nums.len());

    // must use into_iter instead of iter here or sum and product don't work
    let combos = nums.into_iter().combinations(k);

    for combo in combos {
        let sum = combo.iter().sum::<u32>();

        if sum == 2020 {
            println!("{:?}: {}", combo, combo.iter().product::<u32>());
        }
    }
}
