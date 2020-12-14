use std::fs::File;
use std::io::Read;

#[derive(PartialEq, Debug, Clone)]
pub enum GridElement {
    Ground,
    Empty,
    Occupied,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Map {
    map: Vec<Vec<GridElement>>,
}

impl Map {
    pub fn step_part1(&self) -> Map {
        let mut new_map = self.clone();

        for (y, line) in self.map.iter().enumerate() {
            for (x, elem) in line.iter().enumerate() {
                match elem {
                    GridElement::Empty => {
                        if self.is_seat_free_to_take_part1(y, x) {
                            new_map.map[y][x] = GridElement::Occupied;
                        }
                    }
                    GridElement::Occupied => {
                        if self.is_seat_too_crowded_part1(y, x) {
                            new_map.map[y][x] = GridElement::Empty;
                        }
                    }
                    _ => (),
                }
            }
        }

        new_map
    }

    pub fn is_seat_free_to_take_part1(&self, y: usize, x: usize) -> bool {
        self.map[y - 1][x - 1] != GridElement::Occupied
            && self.map[y - 1][x] != GridElement::Occupied
            && self.map[y - 1][x + 1] != GridElement::Occupied
            && self.map[y][x - 1] != GridElement::Occupied
            && self.map[y][x + 1] != GridElement::Occupied
            && self.map[y + 1][x - 1] != GridElement::Occupied
            && self.map[y + 1][x] != GridElement::Occupied
            && self.map[y + 1][x + 1] != GridElement::Occupied
    }

    pub fn is_seat_too_crowded_part1(&self, y: usize, x: usize) -> bool {
        self.map[y - 1..=y + 1]
            .iter()
            .map(|line| {
                line[x - 1..=x + 1]
                    .iter()
                    .filter(|elem| **elem == GridElement::Occupied)
                    .count()
            })
            .sum::<usize>()
            >= 5
    }

    pub fn step_part2(&self) -> Map {
        let mut new_map = self.clone();

        for (y, line) in self.map.iter().enumerate() {
            for (x, elem) in line.iter().enumerate() {
                match elem {
                    GridElement::Empty => {
                        if self.is_seat_free_to_take_part2(y, x) {
                            new_map.map[y][x] = GridElement::Occupied;
                        }
                    }
                    GridElement::Occupied => {
                        if self.is_seat_too_crowded_part2(y, x) {
                            new_map.map[y][x] = GridElement::Empty;
                        }
                    }
                    _ => (),
                }
            }
        }

        new_map
    }

    // TODO: PART2
    pub fn is_seat_free_to_take_part2(&self, y: usize, x: usize) -> bool {
        false
    }

    pub fn is_seat_too_crowded_part2(&self, y: usize, x: usize) -> bool {
        self.map[y - 1..=y + 1]
            .iter()
            .map(|line| {
                line[x - 1..=x + 1]
                    .iter()
                    .filter(|elem| **elem == GridElement::Occupied)
                    .count()
            })
            .sum::<usize>()
            >= 6
    }

    pub fn count_empty_seats(&self) -> usize {
        self.map
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|elem| **elem == GridElement::Empty)
                    .count()
            })
            .sum()
    }

    pub fn count_occupied_seats(&self) -> usize {
        self.map
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|elem| **elem == GridElement::Occupied)
                    .count()
            })
            .sum()
    }
}

pub fn parse_input(input: &str) -> Map {
    let mut map = input
        .lines()
        .map(|line| {
            let mut line = line
                .chars()
                .map(|c| match c {
                    '.' => GridElement::Ground,
                    'L' => GridElement::Empty,
                    '#' => GridElement::Occupied,
                    _ => panic!(),
                })
                .collect::<Vec<GridElement>>();
            line.insert(0, GridElement::Ground);
            line.push(GridElement::Ground);

            line
        })
        .collect::<Vec<Vec<GridElement>>>();
    map.insert(0, vec![GridElement::Ground; map[0].len()]);
    map.push(vec![GridElement::Ground; map[0].len()]);

    Map { map }
}

pub fn solve_part1(input: &str) -> usize {
    let mut map = parse_input(input);
    loop {
        let next_map = map.step_part1();
        if next_map == map {
            return map.count_occupied_seats();
        }
        map = next_map;
    }
}

pub fn solve_part2(input: &str) -> usize {
    let mut map = parse_input(input);
    loop {
        let next_map = map.step_part2();
        if next_map == map {
            return map.count_occupied_seats();
        }
        map = next_map;
    }
}

pub fn part1() {
    let mut file = File::open("input/2020/day11.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&input));
}

pub fn part2() {
    let mut file = File::open("input/2020/day11.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_step_example() {
        assert_eq!(parse_input("L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL").step_part1(), parse_input("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##"));
        assert_eq!(parse_input("#.##.##.##\n#######.##\n#.#.#..#..\n####.##.##\n#.##.##.##\n#.#####.##\n..#.#.....\n##########\n#.######.#\n#.#####.##").step_part1(), parse_input("#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##"));
        assert_eq!(parse_input("#.LL.L#.##\n#LLLLLL.L#\nL.L.L..L..\n#LLL.LL.L#\n#.LL.LL.LL\n#.LLLL#.##\n..L.L.....\n#LLLLLLLL#\n#.LLLLLL.L\n#.#LLLL.##").step_part1(), parse_input("#.##.L#.##\n#L###LL.L#\nL.#.#..#..\n#L##.##.L#\n#.##.LL.LL\n#.###L#.##\n..#.#.....\n#L######L#\n#.LL###L.L\n#.#L###.##"));
    }

    #[test]
    fn test_map_step() {
        assert_eq!(
            parse_input("L.\nLL\nL.").step_part1(),
            parse_input("#.\n##\n#.")
        );
        assert_eq!(
            parse_input("L.L\nLLL\nL.L").step_part1(),
            parse_input("#.#\n###\n#.#")
        );
    }

    #[test]
    fn test_map_seat_is_free_to_take() {
        // NOTE: the map is offset by 1 in X and Y as we add a GridElement::Ground around the map
        let map = parse_input("L.L\nLLL\nL.L");
        assert_eq!(map.is_seat_free_to_take_part1(1, 1), true);
        assert_eq!(map.is_seat_free_to_take_part1(1, 3), true);
        assert_eq!(map.is_seat_free_to_take_part1(2, 1), true);
        assert_eq!(map.is_seat_free_to_take_part1(2, 2), true);
        assert_eq!(map.is_seat_free_to_take_part1(2, 3), true);
        assert_eq!(map.is_seat_free_to_take_part1(3, 1), true);
        assert_eq!(map.is_seat_free_to_take_part1(3, 3), true);
        let map = parse_input("#.L\nLL#\nLL.");
        assert_eq!(map.is_seat_free_to_take_part1(1, 3), false);
        assert_eq!(map.is_seat_free_to_take_part1(2, 1), false);
        assert_eq!(map.is_seat_free_to_take_part1(2, 2), false);
        assert_eq!(map.is_seat_free_to_take_part1(3, 1), true);
        assert_eq!(map.is_seat_free_to_take_part1(3, 3), false);
    }

    #[test]
    fn test_map_parse() {
        assert_eq!(
            parse_input(".L\nL#\nL."),
            Map {
                map: vec![
                    vec![
                        GridElement::Ground,
                        GridElement::Ground,
                        GridElement::Ground,
                        GridElement::Ground
                    ],
                    vec![
                        GridElement::Ground,
                        GridElement::Ground,
                        GridElement::Empty,
                        GridElement::Ground
                    ],
                    vec![
                        GridElement::Ground,
                        GridElement::Empty,
                        GridElement::Occupied,
                        GridElement::Ground
                    ],
                    vec![
                        GridElement::Ground,
                        GridElement::Empty,
                        GridElement::Ground,
                        GridElement::Ground
                    ],
                    vec![
                        GridElement::Ground,
                        GridElement::Ground,
                        GridElement::Ground,
                        GridElement::Ground
                    ],
                ]
            }
        )
    }
}
