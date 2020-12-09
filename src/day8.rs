use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Debug, Clone)]
pub enum Opcode {
    Jmp,
    Acc,
    Nop,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Instruction {
    op: Opcode,
    param: i32,
}

pub fn solve_part1(instructions: &[Instruction]) -> i32 {
    let mut pc = 0;
    let mut acc = 0;
    let mut already_viewed_instructions: HashSet<i32> = HashSet::new();

    while !already_viewed_instructions.contains(&pc) {
        already_viewed_instructions.insert(pc);
        let current_instr = &instructions[pc as usize];

        pc += 1;
        match current_instr.op {
            Opcode::Jmp => pc += current_instr.param - 1,
            Opcode::Nop => (),
            Opcode::Acc => acc += current_instr.param,
        }
    }

    acc
}

pub fn solve_part2(instructions: &[Instruction]) -> i32 {
    for (i, instruction) in instructions.iter().enumerate() {
        match instruction.op {
            Opcode::Jmp => {
                let mut instructions = instructions.to_owned();
                instructions[i].op = Opcode::Nop;
                if let Some(acc) = run_part2(&instructions) {
                    println!("instruction was a JMP at {}", i);
                    return acc;
                }
            }
            Opcode::Nop => {
                let mut instructions = instructions.to_owned();
                instructions[i].op = Opcode::Jmp;
                if let Some(acc) = run_part2(&instructions) {
                    println!("instruction was a NOP at {}", i);
                    return acc;
                }
            }
            _ => (),
        }
    }

    unreachable!();
}

fn run_part2(instructions: &[Instruction]) -> Option<i32> {
    let mut pc = 0;
    let mut acc = 0;
    let mut already_viewed_instructions: HashSet<i32> = HashSet::new();

    loop {
        if already_viewed_instructions.contains(&pc) {
            return None;
        }

        already_viewed_instructions.insert(pc);
        let current_instr = &instructions[pc as usize];

        pc += 1;
        match current_instr.op {
            Opcode::Jmp => pc += current_instr.param - 1,
            Opcode::Nop => (),
            Opcode::Acc => acc += current_instr.param,
        }

        if pc as usize >= instructions.len() {
            return Some(acc);
        }
    }
}

pub fn parse_part1(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(
            |line| match line.split(' ').collect::<Vec<_>>().as_slice() {
                [op, param] => Instruction {
                    op: match *op {
                        "jmp" => Opcode::Jmp,
                        "acc" => Opcode::Acc,
                        "nop" => Opcode::Nop,
                        _ => panic!(),
                    },
                    param: param.parse().unwrap(),
                },
                _ => panic!(),
            },
        )
        .collect()
}

pub fn part1() {
    let mut file = File::open("input/2020/day8.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day8.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&parse_part1(&input)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_example() {
        assert_eq!(
            solve_part1(&parse_part1(
                "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"
            )),
            5
        );
    }

    #[test]
    fn parse_part1_example() {
        assert_eq!(
            parse_part1("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6"),
            vec![
                Instruction {
                    op: Opcode::Nop,
                    param: 0,
                },
                Instruction {
                    op: Opcode::Acc,
                    param: 1,
                },
                Instruction {
                    op: Opcode::Jmp,
                    param: 4,
                },
                Instruction {
                    op: Opcode::Acc,
                    param: 3,
                },
                Instruction {
                    op: Opcode::Jmp,
                    param: -3,
                },
                Instruction {
                    op: Opcode::Acc,
                    param: -99,
                },
                Instruction {
                    op: Opcode::Acc,
                    param: 1,
                },
                Instruction {
                    op: Opcode::Jmp,
                    param: -4,
                },
                Instruction {
                    op: Opcode::Acc,
                    param: 6,
                },
            ]
        )
    }
}
