use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

pub fn parse_input(input: &str) -> HashSet<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part1(input: &HashSet<usize>) -> usize {
    let mut current_jolt = 0;
    let mut diff_map: [usize; 3] = [0; 3];
    while &current_jolt != input.iter().max().unwrap() {
        match find_adapter_for_jolt(input, current_jolt) {
            Some((diff, next_jolt)) => {
                current_jolt = next_jolt;
                diff_map[diff - 1] += 1;
            }
            None => panic!("failed to find adapter for jolt {}", current_jolt),
        }
    }
    diff_map[0] * (diff_map[2] + 1)
}

pub fn find_adapter_for_jolt(adapters: &HashSet<usize>, jolt: usize) -> Option<(usize, usize)> {
    (1..=3).find_map(|diff| Some((diff, *adapters.get(&(jolt + diff))?)))
}

pub fn part1() {
    let mut file = File::open("input/2020/day10.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_input(&input)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_adapter_for_jolt() {
        let mut adapters = HashSet::new();
        adapters.insert(3);
        adapters.insert(6);

        assert_eq!(find_adapter_for_jolt(&adapters, 3), Some((3, 6)));
    }
}
