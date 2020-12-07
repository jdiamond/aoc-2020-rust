use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    color: String,
    contents: Vec<Content>,
}

#[derive(Debug)]
struct Content {
    qty: usize,
    color: String,
}

pub fn part1(input: &str) -> String {
    let rules = parse(input);

    // dbg!(&rules);

    let map = rules
        .iter()
        .map(|rule| (rule.color.clone(), rule))
        .collect::<HashMap<_, _>>();

    // dbg!(&map);

    let results = rules
        .iter()
        .filter(|rule| bag_contains(&map, &(*rule).color, "shiny gold"))
        .collect::<Vec<&Rule>>();

    // dbg!(&results);

    String::from(results.len().to_string())
}

fn bag_contains(map: &HashMap<String, &Rule>, outer: &str, inner: &str) -> bool {
    let rule = &map[outer];

    for c in rule.contents.iter() {
        if c.color == inner || bag_contains(map, &c.color, inner) {
            return true;
        }
    }

    false
}

pub fn part2(input: &str) -> String {
    let rules = parse(input);

    // dbg!(&rules);

    let map = rules
        .iter()
        .map(|rule| (rule.color.clone(), rule))
        .collect::<HashMap<_, _>>();

    // dbg!(&map);

    String::from((count_inners(&map, "shiny gold") - 1).to_string())
}

fn count_inners(map: &HashMap<String, &Rule>, outer: &str) -> usize {
    let outer = map[outer];

    1 + outer
        .contents
        .iter()
        .map(|c| c.qty * count_inners(map, &c.color))
        .sum::<usize>()
}

fn parse(input: &str) -> Vec<Rule> {
    let re = Regex::new(r"(\w+ \w+) bags contain (.+)\.").unwrap();
    let re2 = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    let rules = input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .map(|cap| {
            let contents = cap.get(2).unwrap().as_str().to_owned();
            let contents = if contents == "no other bags" {
                vec![]
            } else {
                contents
                    .split(", ")
                    .map(|s| {
                        let cap = re2.captures(s).unwrap();

                        Content {
                            qty: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                            color: cap.get(2).unwrap().as_str().to_owned(),
                        }
                    })
                    .collect::<Vec<_>>()
            };

            Rule {
                color: cap.get(1).unwrap().as_str().to_owned(),
                contents: contents,
            }
        })
        .collect::<Vec<_>>();

    rules
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day7-example.txt");

        assert_eq!(part1(&input), "4");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day7-puzzle.txt");

        assert_eq!(part1(&input), "213");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day7-example2.txt");

        assert_eq!(part2(&input), "126");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day7-puzzle.txt");

        assert_eq!(part2(&input), "38426");
    }
}
