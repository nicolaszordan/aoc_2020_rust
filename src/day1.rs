use itertools::Itertools;
use std::fs::File;
use std::io::Read;

fn solve_part1(input: &[u32]) -> u32 {
    for (index, num1) in input.iter().enumerate() {
        for num2 in input[index..].iter() {
            if num1 + num2 == 2020 {
                return num1 * num2;
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn solve_part1_cartesian_product(input: &[u32]) -> u32 {
    let (l, r) = input
        .iter()
        .cartesian_product(input)
        .find(|(l, r)| *l + *r == 2020)
        .unwrap();

    l * r
}

fn solve_part2(input: &[u32]) -> u32 {
    for (index1, num1) in input.iter().enumerate() {
        for (index2, num2) in input[index1..].iter().enumerate() {
            for num3 in input[index1 + index2..].iter() {
                if num1 + num2 + num3 == 2020 {
                    return num1 * num2 * num3;
                }
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn solve_part2_cartesian_product(input: &[u32]) -> u32 {
    let ((l, m), r) = input
        .iter()
        .cartesian_product(input)
        .cartesian_product(input)
        .find(|((l, m), r)| *l + *m + *r == 2020)
        .unwrap();

    l * m * r
}

fn parse_part1(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn part1() {
    let mut file = File::open("input/2020/day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    // part1 and part2 use the same generator
    println!("{}", solve_part2(&parse_part1(&input)));
}

#[cfg(test)]
mod test {
    use super::parse_part1;
    use super::solve_part1;
    use super::solve_part2;

    #[test]
    fn solve_day1_part1_example() {
        assert_eq!(solve_part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn parse_day1_part1_example() {
        assert_eq!(
            parse_part1("123\n456\n789\n0\n2"),
            [123, 456, 789, 0, 2].to_vec()
        );
    }

    #[test]
    fn solve_day1_part2_example() {
        assert_eq!(solve_part2(&[1721, 979, 366, 299, 675, 1456]), 241861950);
    }
}
