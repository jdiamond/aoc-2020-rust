use std::io::{self, BufRead};
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    let stdin = io::stdin();
    let mut nums = Vec::new();

    for line in stdin.lock().lines() {
        let num = u32::from_str(&line.unwrap()).unwrap();
        nums.push(num);
    }

    println!("got {} nums", nums.len());

    for (index1, num1) in nums.iter().enumerate() {
        for (index2, num2) in nums.iter().enumerate() {
            if index1 < index2 {
                let sum = num1 + num2;

                if sum == 2020 {
                    let prod = num1 * num2;

                    println!("{} * {} = {}", num1, num2, prod);
                }
            }
        }
    }
}
