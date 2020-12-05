use regex::Regex;
use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Clone, Debug)]
struct PasswordRequirement {
    lower_limit: usize, // min. number of times `required_char` must be present in `password`
    upper_limit: usize, // max. number of times `required_char` must be present in `password`
    required_char: char,
    password: String,
}

fn solve_part1(password_requirements: &[PasswordRequirement]) -> usize {
    password_requirements
        .iter()
        .filter(|req| {
            let count = req
                .password
                .chars()
                .filter(|c| *c == req.required_char)
                .count();
            count >= req.lower_limit && count <= req.upper_limit
        })
        .count()
}

fn solve_part2(password_requirements: &[PasswordRequirement]) -> usize {
    password_requirements
        .iter()
        .filter(|req| {
            (req.password.chars().nth(req.lower_limit - 1).unwrap() == req.required_char)
                != (req.password.chars().nth(req.upper_limit - 1).unwrap() == req.required_char)
        })
        .count()
}

fn parse_part1(input: &str) -> Vec<PasswordRequirement> {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            PasswordRequirement {
                lower_limit: caps.get(1).unwrap().as_str().parse().unwrap(),
                upper_limit: caps.get(2).unwrap().as_str().parse().unwrap(),
                required_char: caps.get(3).unwrap().as_str().chars().next().unwrap(),
                password: caps.get(4).unwrap().as_str().to_owned(),
            }
        })
        .collect()
}

pub fn part1() {
    let mut file = File::open("input/2020/day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day2.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&parse_part1(&input)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_day2_part1_example() {
        assert_eq!(
            solve_part1(&[
                PasswordRequirement {
                    lower_limit: 1,
                    upper_limit: 3,
                    required_char: 'a',
                    password: "abcde".to_owned()
                },
                PasswordRequirement {
                    lower_limit: 1,
                    upper_limit: 3,
                    required_char: 'b',
                    password: "cdefg".to_owned()
                },
                PasswordRequirement {
                    lower_limit: 2,
                    upper_limit: 9,
                    required_char: 'c',
                    password: "cccccccc".to_owned()
                },
            ]),
            2
        );
    }

    #[test]
    fn solve_day2_part2_example() {
        assert_eq!(
            solve_part1(&[
                PasswordRequirement {
                    lower_limit: 1,
                    upper_limit: 3,
                    required_char: 'a',
                    password: "abcde".to_owned()
                },
                PasswordRequirement {
                    lower_limit: 1,
                    upper_limit: 3,
                    required_char: 'b',
                    password: "cdefg".to_owned()
                },
                PasswordRequirement {
                    lower_limit: 2,
                    upper_limit: 9,
                    required_char: 'c',
                    password: "cccccccc".to_owned()
                },
            ]),
            1
        );
    }

    #[test]
    fn parse_day2_part1_example() {
        assert_eq!(
            parse_part1("1-3 a: abcde\n1-3 b: cdefg\n19-49 c: cccccccc\n\n"),
            [
                PasswordRequirement {
                    lower_limit: 1,
                    upper_limit: 3,
                    required_char: 'a',
                    password: "abcde".to_owned()
                },
                PasswordRequirement {
                    lower_limit: 1,
                    upper_limit: 3,
                    required_char: 'b',
                    password: "cdefg".to_owned()
                },
                PasswordRequirement {
                    lower_limit: 19,
                    upper_limit: 49,
                    required_char: 'c',
                    password: "cccccccc".to_owned()
                },
            ]
            .to_vec()
        );
    }
}
