use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

pub fn part1(input: &str) -> String {
    let code = parse(input);

    // dbg!(&code);

    let acc = run(&code);

    acc.to_string()
}

pub fn part2(_input: &str) -> String {
    String::from("")
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match &line[..3] {
            "acc" => Instruction::Acc(line[4..].parse::<isize>().unwrap()),
            "jmp" => Instruction::Jmp(line[4..].parse::<isize>().unwrap()),
            "nop" => Instruction::Nop,
            _ => panic!(),
        })
        .collect::<Vec<_>>()
}

fn run(code: &Vec<Instruction>) -> isize {
    let mut acc = 0 as isize;
    let mut ip = 0 as usize;
    let mut seen = HashSet::<usize>::new();

    loop {
        let instr = &code[ip];

        // println!("{} {:?} {}", ip, instr, acc);

        if seen.contains(&ip) {
            break;
        }

        seen.insert(ip);

        match *instr {
            Instruction::Acc(val) => acc += val,
            Instruction::Jmp(val) => {
                ip = (if val.is_negative() {
                    ip.checked_sub(val.wrapping_abs() as u32 as usize)
                } else {
                    ip.checked_add(val as usize)
                })
                .unwrap();
                continue;
            }
            Instruction::Nop => {}
        }

        ip += 1;

        if ip >= code.len() {
            break;
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::load_input;

    #[test]
    fn part1_example_works() {
        let input = load_input("day8-example.txt");

        assert_eq!(part1(&input), "5");
    }

    #[test]
    fn part1_input_works() {
        let input = load_input("day8-puzzle.txt");

        assert_eq!(part1(&input), "1501");
    }

    #[test]
    fn part2_example_works() {
        let input = load_input("day8-example.txt");

        assert_eq!(part2(&input), "");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day8-puzzle.txt");

        assert_eq!(part2(&input), "");
    }
}
