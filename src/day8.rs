use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

pub fn part1(input: &str) -> String {
    let code = parse(input);

    // dbg!(&code);

    let (acc, _) = run(&code);

    acc.to_string()
}

pub fn part2(input: &str) -> String {
    let mut code = parse(input);
    let mut ip = 0 as usize;
    let mut acc = 0 as isize;

    loop {
        match code[ip] {
            Instruction::Acc(_) => {}
            Instruction::Jmp(val) => {
                code[ip] = Instruction::Nop(val);

                let (acc2, infinite) = run(&code);

                if !infinite {
                    acc = acc2;
                    break;
                } else {
                    code[ip] = Instruction::Jmp(val);
                }
            }
            Instruction::Nop(val) => {
                if val != 0 {
                    code[ip] = Instruction::Jmp(val);

                    let (acc2, infinite) = run(&code);

                    if !infinite {
                        acc = acc2;
                        break;
                    } else {
                        code[ip] = Instruction::Nop(val);
                    }
                }
            }
        }

        ip += 1;

        if ip >= code.len() {
            break;
        }
    }

    acc.to_string()
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match &line[..3] {
            "acc" => Instruction::Acc(line[4..].parse::<isize>().unwrap()),
            "jmp" => Instruction::Jmp(line[4..].parse::<isize>().unwrap()),
            "nop" => Instruction::Nop(line[4..].parse::<isize>().unwrap()),
            _ => panic!(),
        })
        .collect::<Vec<_>>()
}

fn run(code: &Vec<Instruction>) -> (isize, bool) {
    let mut acc = 0 as isize;
    let mut ip = 0 as usize;
    let mut seen = HashSet::<usize>::new();
    let mut infinite = false;

    loop {
        if ip >= code.len() {
            break;
        }

        let instr = &code[ip];

        // println!("{} {:?} {}", ip, instr, acc);

        if seen.contains(&ip) {
            infinite = true;
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
            Instruction::Nop(_) => {}
        }

        ip += 1;
    }

    (acc, infinite)
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

        assert_eq!(part2(&input), "8");
    }

    #[test]
    fn part2_input_works() {
        let input = load_input("day8-puzzle.txt");

        assert_eq!(part2(&input), "509");
    }
}
