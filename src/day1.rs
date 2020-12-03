use itertools::Itertools;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::vec::Vec;

pub fn part1() {
    let stdin = io::stdin();
    let mut nums = Vec::new();

    for line in stdin.lock().lines() {
        let num = u32::from_str(&line.unwrap()).unwrap();
        nums.push(num);
    }

    println!("got {} nums", nums.len());

    let combos = nums.iter().combinations(2);

    for combo in combos {
        let num1 = combo[0];
        let num2 = combo[1];
        let sum = num1 + num2;

        if sum == 2020 {
            let prod = num1 * num2;

            println!("{} * {} = {}", num1, num2, prod);
        }
    }
}

pub fn part2() {
    let stdin = io::stdin();
    let mut nums = Vec::new();

    for line in stdin.lock().lines() {
        let num = u32::from_str(&line.unwrap()).unwrap();
        nums.push(num);
    }

    println!("got {} nums", nums.len());

    let combos = nums.iter().combinations(3);

    for combo in combos {
        let num1 = combo[0];
        let num2 = combo[1];
        let num3 = combo[2];
        let sum = num1 + num2 + num3;

        if sum == 2020 {
            let prod = num1 * num2 * num3;

            println!("{} * {} * {} = {}", num1, num2, num3, prod);
        }
    }
}
