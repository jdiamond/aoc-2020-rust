pub fn part1(input: &str) -> String {
    input.lines().map(decode).max().unwrap().to_string()
}

pub fn part2(input: &str) -> String {
    let mut seats = input.lines().map(decode).collect::<Vec<_>>();

    seats.sort();

    find_first_missing(seats).unwrap().to_string()
}

fn decode(seat: &str) -> usize {
    let row = &seat[0..7].replace("F", "0").replace("B", "1");
    let row = usize::from_str_radix(row, 2).unwrap();

    let col = &seat[7..].replace("L", "0").replace("R", "1");
    let col = usize::from_str_radix(col, 2).unwrap();

    row * 8 + col
}

fn find_first_missing(seats: Vec<usize>) -> Option<usize> {
    let mut last = seats[0] - 1;

    for seat in seats {
        if seat - 1 > last {
            return Some(seat - 1);
        }

        last = seat;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn decode_works() {
        assert_eq!(decode("BFFFBBFRRR"), 567);
        assert_eq!(decode("FFFBBBFRRR"), 119);
        assert_eq!(decode("BBFFBBFRLL"), 820);
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day5-puzzle.txt");

        assert_eq!(part1(&input), "976");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day5-puzzle.txt");

        assert_eq!(part2(&input), "685");
    }
}
