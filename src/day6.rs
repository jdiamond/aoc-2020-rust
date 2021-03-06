use itertools::Itertools;
use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let votes = input
        .split("\n\n")
        .map(|group| group.lines().flat_map(|line| line.chars()).unique().count())
        .sum::<usize>();

    String::from(votes.to_string())
}

pub fn part2(input: &str) -> String {
    let votes = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold1(|acc, cur| acc.intersection(&cur).copied().collect())
                .unwrap()
        })
        .map(|group| group.len())
        .sum::<usize>();

    String::from(votes.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day6-example.txt");

        assert_eq!(part1(&input), "11");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day6-puzzle.txt");

        assert_eq!(part1(&input), "6532");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day6-example.txt");

        assert_eq!(part2(&input), "6");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day6-puzzle.txt");

        assert_eq!(part2(&input), "3427");
    }
}
