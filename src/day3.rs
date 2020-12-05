use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Clone, Debug)]
enum GridElement {
    Empty,
    Tree,
}

fn solve_part1(map: &[Vec<GridElement>]) -> usize {
    tree_in_slope(map, 3, 1)
}

fn solve_part2(map: &[Vec<GridElement>]) -> usize {
    tree_in_slope(map, 1, 1)
        * tree_in_slope(map, 3, 1)
        * tree_in_slope(map, 5, 1)
        * tree_in_slope(map, 7, 1)
        * tree_in_slope(map, 1, 2)
}

fn tree_in_slope(map: &[Vec<GridElement>], vx: usize, vy: usize) -> usize {
    let max_x = map[0].len();
    let mut tree_count = 0;
    let mut pos_x = 0;
    for line in map.iter().step_by(vy) {
        if line[pos_x % max_x] == GridElement::Tree {
            tree_count += 1;
        }
        pos_x += vx;
    }

    tree_count
}

fn parse_part1(input: &str) -> Vec<Vec<GridElement>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => GridElement::Empty,
                    '#' => GridElement::Tree,
                    _ => unimplemented!(),
                })
                .collect()
        })
        .collect()
}

pub fn part1() {
    let mut file = File::open("input/2020/day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_part1(&input)));
}

pub fn part2() {
    let mut file = File::open("input/2020/day3.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&parse_part1(&input)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_day3_part1_example() {
        assert_eq!(
            solve_part1(&vec![
                vec![
                    GridElement::Empty,
                    GridElement::Empty,
                    GridElement::Tree,
                    GridElement::Tree
                ],
                vec![
                    GridElement::Empty,
                    GridElement::Empty,
                    GridElement::Empty,
                    GridElement::Tree
                ],
                vec![
                    GridElement::Empty,
                    GridElement::Empty,
                    GridElement::Tree,
                    GridElement::Empty
                ],
            ]),
            2
        );
    }

    #[test]
    fn parse_day3_part1_example() {
        assert_eq!(
            parse_part1("..##\n#...\n.#..\n"),
            vec![
                vec![
                    GridElement::Empty,
                    GridElement::Empty,
                    GridElement::Tree,
                    GridElement::Tree
                ],
                vec![
                    GridElement::Tree,
                    GridElement::Empty,
                    GridElement::Empty,
                    GridElement::Empty
                ],
                vec![
                    GridElement::Empty,
                    GridElement::Tree,
                    GridElement::Empty,
                    GridElement::Empty
                ],
            ]
        );
    }
}
