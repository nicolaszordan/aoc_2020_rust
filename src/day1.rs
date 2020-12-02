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
    0
}

fn generate_part1(input: &str) -> Vec<u32> {
    input.lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

pub fn part1() {
    let mut file = File::open("input/2020/day1.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&generate_part1(&input)));
}

mod test {
    use super::solve_part1;
    use super::generate_part1;

    #[test]
    fn solve_example() {
        assert_eq!(solve_part1(&[1721, 979, 366, 299, 675, 1456]), 514579);
    }

    #[test]
    fn generate_example() {
        assert_eq!(generate_part1("123\n456\n789\n0\n2"), [123, 456, 789, 0, 2].to_vec());
    }
}