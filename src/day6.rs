use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn solve_part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_ascii_lowercase())
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

pub fn solve_part1_iter_over_alpha(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| ('a'..'z').filter(|c| group.contains(|g| *c == g)).count())
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let group = group
                .lines()
                .map(|person| person.to_owned())
                .collect::<Vec<String>>();
            group[0]
                .chars()
                .filter(|c| {
                    group
                        .iter()
                        .skip(1)
                        .all(|person| person.contains(|p| *c == p))
                })
                .count()
        })
        .sum()
}

pub fn solve_part2_iter_over_alpha(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            let nbperson = group.lines().count();
            ('a'..'z')
                .filter(|c| group.matches(|g| *c == g).count() == nbperson)
                .count()
        })
        .sum()
}

pub fn part1() {
    let mut file = File::open("input/2020/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&input));
}

pub fn part2() {
    let mut file = File::open("input/2020/day6.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_example() {
        assert_eq!(
            solve_part1("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"),
            11
        );
    }
}
