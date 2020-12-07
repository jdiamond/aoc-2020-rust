pub fn part1(_input: &str) -> String {
    String::from("")
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
        let input = load_input("day7-example.txt");

        assert_eq!(part1(&input), "");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day7-puzzle.txt");

        assert_eq!(part1(&input), "");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day7-example.txt");

        assert_eq!(part2(&input), "");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day7-puzzle.txt");

        assert_eq!(part2(&input), "");
    }
}
