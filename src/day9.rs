use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

pub fn parse_part1(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part1(input: &[usize]) -> usize {
    crack_xmas(input, 25).unwrap()
}

pub fn crack_xmas(input: &[usize], preamble: usize) -> Option<usize> {
    for (i, _) in input.iter().enumerate() {
        // will panic if input[i+preamble] is OOB
        if found_vulnerability(&input[i..i + preamble], input[i + preamble]) {
            return Some(input[i + preamble]);
        }
    }
    None
}

fn found_vulnerability(input: &[usize], must_add_to: usize) -> bool {
    input
        .iter()
        .cartesian_product(input)
        .find(|(l, r)| *l + *r == must_add_to)
        .is_none()
}

pub fn solve_part2_paul_b(input: &[usize], to_find: usize) -> usize {
    let mut lower = 0;
    let mut upper = 2;
    loop {
        match input[lower..upper].iter().sum::<usize>().cmp(&to_find) {
            Ordering::Equal => {
                return input[lower..upper].iter().max().unwrap()
                    + input[lower..upper].iter().min().unwrap()
            }
            Ordering::Less => upper += 1,
            Ordering::Greater => lower += 1,
        }
    }
}

pub fn part1() {
    let mut file = File::open("input/2020/day9.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day9.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2_paul_b(&parse_part1(&input), 10884537));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_example() {
        assert_eq!(crack_xmas(&parse_part1("35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"), 5), Some(127));
    }

    #[test]
    fn parse_part1_example() {
        assert_eq!(parse_part1("35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"), vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]);
    }

    #[test]
    fn solve_part2_paul_b_example() {
        assert_eq!(solve_part2_paul_b(&parse_part1("35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"), 127), 62);
    }
}
