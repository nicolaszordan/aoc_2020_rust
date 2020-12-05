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

pub fn part1() {
    let mut file = File::open("input/2020/day5.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&input));
}

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
