use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: cargo run <day> <part> < input.txt");
    } else {
        let day = &args[1];
        let part = &args[2];

        println!("running day {} part {}", day, part);

        if day == "1" && part == "1" {
            day1::part1();
        } else if day == "1" && part == "2" {
            day1::part2();
        }
    }
}
