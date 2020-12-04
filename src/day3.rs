type TreeMap = Vec<Vec<bool>>;

pub fn part1(input: &str) -> String {
    let map = parse_input(input);
    let hit = run(&map, 3, 1);
    hit.to_string()
}

pub fn part2(input: &str) -> String {
    let map = parse_input(input);
    let answer =
        run(&map, 1, 1) * run(&map, 3, 1) * run(&map, 5, 1) * run(&map, 7, 1) * run(&map, 1, 2);
    answer.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<TreeMap>()
}

fn run(map: &TreeMap, right: usize, down: usize) -> usize {
    let mut x = 0;
    let mut y = 0;

    let width = map[0].len();
    let height = map.len();

    let mut hit = 0;

    while y < (height - 1) {
        // dbg!(x, y);

        x += right;
        x %= width;
        y += down;

        if map[y][x] {
            hit += 1;
        }
    }

    hit
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day3-example.txt");

        assert_eq!(part1(&input), "7");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day3-puzzle.txt");

        assert_eq!(part1(&input), "247");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day3-example.txt");

        assert_eq!(part2(&input), "336");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day3-puzzle.txt");

        assert_eq!(part2(&input), "2983070376");
    }
}
