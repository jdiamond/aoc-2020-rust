use itertools::Itertools;

pub fn part1(input: &str) -> String {
    run(input, 2)
}

pub fn part2(input: &str) -> String {
    run(input, 3)
}

fn run(input: &str, k: usize) -> String {
    let result = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .combinations(k)
        .find(|combo| combo.iter().sum::<u32>() == 2020)
        .unwrap();

    let product = result.iter().product::<u32>();

    println!("{:?}: {}", result, product);

    product.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day1-example.txt");

        assert_eq!(part1(&input), "514579");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day1-puzzle.txt");

        assert_eq!(part1(&input), "1013211");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day1-example.txt");

        assert_eq!(part2(&input), "241861950");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day1-puzzle.txt");

        assert_eq!(part2(&input), "13891280");
    }
}
