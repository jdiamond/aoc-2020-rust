use itertools::Itertools;

pub fn part1(input: &str) {
    run(input, 2);
}

pub fn part2(input: &str) {
    run(input, 3);
}

fn run(input: &str, k: usize) {
    let results = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .combinations(k)
        .filter(|combo| combo.iter().sum::<u32>() == 2020);

    for result in results {
        println!("{:?}: {}", result, result.iter().product::<u32>());
    }
}
