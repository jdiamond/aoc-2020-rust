use itertools::Itertools;
use itertools::MinMaxResult::MinMax;

pub fn part1(input: &str) -> String {
    let nums = parse(input);
    let size = nums[0];
    let first_invalid = find_first_invalid(&nums, size);

    first_invalid.to_string()
}

pub fn part2(input: &str) -> String {
    let nums = parse(input);
    let size = nums[0];
    let first_invalid = find_first_invalid(&nums, size);
    let weakness = find_weakness(&nums, first_invalid);

    weakness.to_string()
}

fn find_weakness(nums: &Vec<usize>, sum: usize) -> usize {
    for (start, num) in nums.iter().enumerate() {
        let mut x = *num;

        for (end, next) in nums.iter().enumerate().skip(start + 1) {
            x += *next;

            if x == sum {
                return match nums[start..end + 1].iter().minmax() {
                    MinMax(min, max) => min + max,
                    _ => 0,
                };
            } else if x > sum {
                break;
            }
        }
    }

    0
}

fn find_first_invalid(nums: &Vec<usize>, size: usize) -> usize {
    let mut index = size + 1;

    loop {
        if !is_valid(&nums, size, index) {
            break;
        }

        index += 1;
    }

    nums[index]
}

fn is_valid(nums: &Vec<usize>, size: usize, index: usize) -> bool {
    let current = nums[index];

    nums[index - size..index]
        .iter()
        .combinations(2)
        .any(|combo| combo[0] + combo[1] == current)
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

        assert_eq!(part2(&input), "62");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day9-puzzle.txt");

        assert_eq!(part2(&input), "219202240");
    }
}
