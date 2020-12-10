use itertools::Itertools;

pub fn part1(input: &str) -> String {
    let nums = parse(input);

    let size = nums[0];
    let mut index = size + 1;

    loop {
        if !is_valid(&nums, size, index) {
            break;
        }

        index += 1;
    }

    nums[index].to_string()
}

fn is_valid(nums: &Vec<usize>, size: usize, index: usize) -> bool {
    let current = nums[index];

    nums[index - size..index]
        .iter()
        .combinations(2)
        .any(|combo| combo[0] + combo[1] == current)
}

pub fn part2(_input: &str) -> String {
    String::from("")
}

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day9-example.txt");

        assert_eq!(part1(&input), "127");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day9-puzzle.txt");

        assert_eq!(part1(&input), "1639024365");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day9-example.txt");

        assert_eq!(part2(&input), "");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day9-puzzle.txt");

        assert_eq!(part2(&input), "");
    }
}
