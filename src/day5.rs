use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!(),
                })
                .fold(0, |acc, bit| acc << 1 | bit)
        })
        .max()
        .unwrap()
}

pub fn solve_part2(input: &str) -> u32 {
    let bording_pass_list = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!(),
                })
                .fold(0, |acc, bit| acc << 1 | bit)
        })
        .collect::<HashSet<u32>>();

    (0..0b1111111111)
        .find(|n| {
            !bording_pass_list.contains(&n)
                && bording_pass_list.contains(&(n + 1))
                && bording_pass_list.contains(&(n - 1))
        })
        .unwrap()
}

#[allow(dead_code)]
pub fn solve_part2_with_min_max(input: &str) -> u32 {
    let mut min = 0b1111111111;
    let mut max = 0;
    let bording_pass_list = input
        .lines()
        .map(|line| {
            let num = line
                .chars()
                .map(|c| match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!(),
                })
                .fold(0, |acc, bit| acc << 1 | bit);
            if min > num {
                min = num;
            }
            if max < num {
                max = num;
            }
            num
        })
        .collect::<HashSet<u32>>();

    (min..max)
        .find(|n| {
            !bording_pass_list.contains(&n)
                && bording_pass_list.contains(&(n + 1))
                && bording_pass_list.contains(&(n - 1))
        })
        .unwrap()
}

pub fn solve_part2_vector(input: &str) -> u32 {
    let bording_pass_list = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'F' | 'L' => 0,
                    'B' | 'R' => 1,
                    _ => panic!(),
                })
                .fold(0, |acc, bit| acc << 1 | bit)
        })
        .collect::<Vec<u32>>();

    (0..0b1111111111)
        .find(|n| {
            !bording_pass_list.contains(&n)
                && bording_pass_list.contains(&(n + 1))
                && bording_pass_list.contains(&(n - 1))
        })
        .unwrap()
}
pub fn part1() {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&input));
}

pub fn part2() {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_example() {
        assert_eq!(solve_part1("FBFBBFFRLR"), 357);
    }

    #[test]
    fn solve_part1_test() {
        assert_eq!(solve_part1("FFFFFFFLLL"), 0);
        assert_eq!(solve_part1("FFFFFFBLLL"), 8);
        assert_eq!(solve_part1("FFFFFFFLLR"), 1);
        assert_eq!(solve_part1("FFFFFFFLRL"), 2);
        assert_eq!(solve_part1("FFFFFFFRLR"), 5);
    }
}
