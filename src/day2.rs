use regex::Regex;

pub fn part1(input: &str) -> String {
    run(input, validate1)
}

pub fn part2(input: &str) -> String {
    run(input, validate2)
}

type Validator = fn(policy: &PasswordPolicy, password: &str) -> bool;

fn run(input: &str, validate: Validator) -> String {
    let records = input.lines().map(|line| parse_line(line));

    let mut total = 0;
    let mut valid = 0;

    for record in records {
        total += 1;

        let is_valid = validate(&record.0, &record.1);

        if is_valid {
            valid += 1;
        }

        // println!("{:?}: {}", record, is_valid);
    }

    println!("there are {} valid passwords out of {} total", valid, total);

    valid.to_string()
}

#[derive(Debug)]
struct PasswordPolicy {
    first: usize,
    second: usize,
    letter: char,
}

fn parse_line(line: &str) -> (PasswordPolicy, String) {
    // println!("{}", line);

    let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();
    let caps = re.captures(line).unwrap();

    // println!("{:?}", caps);

    (
        PasswordPolicy {
            first: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            second: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
        },
        caps.get(4).unwrap().as_str().to_string(),
    )
}

fn validate1(policy: &PasswordPolicy, password: &str) -> bool {
    let count = password.chars().fold(0, |count, char| {
        if char == policy.letter {
            count + 1
        } else {
            count
        }
    });

    count >= policy.first && count <= policy.second
}

fn validate2(policy: &PasswordPolicy, password: &str) -> bool {
    let first = password.chars().nth(policy.first - 1).unwrap();
    let second = password.chars().nth(policy.second - 1).unwrap();

    (first == policy.letter && second != policy.letter)
        || (first != policy.letter && second == policy.letter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate1_works() {
        assert!(validate1(
            &PasswordPolicy {
                first: 1,
                second: 3,
                letter: 'a',
            },
            "abcde",
        ));
    }

    #[test]
    fn validate2_works() {
        assert!(validate2(
            &PasswordPolicy {
                first: 1,
                second: 3,
                letter: 'a',
            },
            "abcde",
        ));
    }

    #[test]
    fn part1_example_works() {
        let input = std::fs::read_to_string("day2-example.txt").unwrap();

        assert_eq!(part1(&input), "2");
    }

    #[test]
    fn part1_input_works() {
        let input = std::fs::read_to_string("day2-input.txt").unwrap();

        assert_eq!(part1(&input), "556");
    }

    #[test]
    fn part2_example_works() {
        let input = std::fs::read_to_string("day2-example.txt").unwrap();

        assert_eq!(part2(&input), "1");
    }

    #[test]
    fn part2_input_works() {
        let input = std::fs::read_to_string("day2-input.txt").unwrap();

        assert_eq!(part2(&input), "605");
    }
}
