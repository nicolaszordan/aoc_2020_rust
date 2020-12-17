use std::fs::File;
use std::io::Read;

pub fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let lines = input.lines().collect::<Vec<&str>>();
    (
        lines[0].parse().unwrap(),
        lines[1]
            .split(',')
            .filter_map(|bus| bus.parse().ok())
            .collect(),
    )
}

pub fn solve_part1(depart: usize, bus_ids: &[usize]) -> usize {
    let bus_infos = bus_ids
        .iter()
        .map(|bus_id| (*bus_id, bus_id - depart % bus_id))
        .min_by(|(_, bus_wait_time_1), (_, bus_wait_time_2)| bus_wait_time_1.cmp(bus_wait_time_2))
        .unwrap();
    bus_infos.0 * bus_infos.1
}

pub fn part1() {
    let mut file = File::open("input/2020/day13.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let parsed_input = parse_input(&input);
    println!("{}", solve_part1(parsed_input.0, &parsed_input.1));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input_example() {
        assert_eq!(
            parse_input("939\n7,13,x,x,59,x,31,19"),
            (939, vec![7, 13, 59, 31, 19])
        );
    }

    #[test]
    fn test_solve_example() {
        let parsed_input = parse_input("939\n7,13,x,x,59,x,31,19");
        assert_eq!(solve_part1(parsed_input.0, &parsed_input.1), 295);
    }
}
