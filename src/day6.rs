use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let votes = input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    // dbg!(&votes);

    let votes = votes.iter().fold(0, |sum, group| sum + group.len());

    String::from(votes.to_string())
}

pub fn part2(_input: &str) -> String {
    String::from("")
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

        assert_eq!(part2(&input), "");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day6-puzzle.txt");

        assert_eq!(part2(&input), "");
    }
}
