pub fn part1(input: &str) -> String {
    run(input)
}

pub fn part2(input: &str) -> String {
    run(input)
}

fn run(input: &str) -> String {
    let map = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    // dbg!(map);

    let right = 3;
    let down = 1;

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

    hit.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_works() {
        let input = std::fs::read_to_string("day3-example.txt").unwrap();

        assert_eq!(part1(&input), "7");
    }

    #[test]
    fn part1_input_works() {
        let input = std::fs::read_to_string("day3-input.txt").unwrap();

        assert_eq!(part1(&input), "247");
    }

    // #[test]
    // fn part2_example_works() {
    //     let input = std::fs::read_to_string("day3-example.txt").unwrap();

    //     assert_eq!(part2(&input), "");
    // }

    // #[test]
    // fn part2_input_works() {
    //     let input = std::fs::read_to_string("day3-input.txt").unwrap();

    //     assert_eq!(part2(&input), "");
    // }
}
