use regex::Regex;
use std::fs::File;
use std::io::Read;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Action {
    MoveNorth,
    MoveSouth,
    MoveEast,
    MoveWest,
    TurnRight,
    TurnLeft,
    MoveForward,
}

#[derive(Debug)]
pub struct Boat {
    angle: i32,
    orientation_y: i32,
    orientation_x: i32,
    position_y: i32,
    position_x: i32,
}

impl Boat {
    pub fn new() -> Boat {
        Boat {
            angle: 90,
            orientation_y: 0,
            orientation_x: 1, // the boat starts facing east
            position_y: 0,
            position_x: 0,
        }
    }

    pub fn get_dist_from_start(&self) -> i32 {
        self.position_x.abs() + self.position_y.abs()
    }

    pub fn do_action(&mut self, action: Action, value: i32) {
        match action {
            Action::MoveNorth => self.move_north(value),
            Action::MoveSouth => self.move_south(value),
            Action::MoveEast => self.move_east(value),
            Action::MoveWest => self.move_west(value),
            Action::TurnLeft => self.turn_left(value),
            Action::TurnRight => self.turn_right(value),
            Action::MoveForward => self.move_forward(value),
        }
    }

    fn move_south(&mut self, value: i32) {
        self.position_y -= value
    }

    fn move_north(&mut self, value: i32) {
        self.position_y += value
    }

    fn move_east(&mut self, value: i32) {
        self.position_x += value
    }

    fn move_west(&mut self, value: i32) {
        self.position_x -= value
    }

    fn turn_left(&mut self, value: i32) {
        self.angle -= value;
        self.orientation_y = (self.angle as f64).to_radians().cos() as i32;
        self.orientation_x = (self.angle as f64).to_radians().sin() as i32;
    }

    fn turn_right(&mut self, value: i32) {
        self.angle += value;
        self.orientation_y = (self.angle as f64).to_radians().cos() as i32;
        self.orientation_x = (self.angle as f64).to_radians().sin() as i32;
    }

    fn move_forward(&mut self, value: i32) {
        self.position_x += self.orientation_x * value;
        self.position_y += self.orientation_y * value;
    }
}

pub fn parse_input(input: &str) -> Vec<(Action, i32)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<action>N|S|E|W|L|R|F)(?P<value>\d+)").unwrap();
    }

    input
        .lines()
        .map(|line| {
            let captures = RE.captures(line).unwrap();
            (
                match &captures["action"] {
                    "N" => Action::MoveNorth,
                    "E" => Action::MoveEast,
                    "S" => Action::MoveSouth,
                    "W" => Action::MoveWest,
                    "L" => Action::TurnLeft,
                    "R" => Action::TurnRight,
                    "F" => Action::MoveForward,
                    _ => panic!(),
                },
                captures["value"].parse().unwrap(),
            )
        })
        .collect()
}

pub fn solve_part1(input: &[(Action, i32)]) -> i32 {
    let mut boat = Boat::new();
    input
        .iter()
        .for_each(|(action, value)| boat.do_action(*action, *value));
    boat.get_dist_from_start()
}

pub fn part1() {
    let mut file = File::open("input/2020/day12.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part1(&parse_input(&input)));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_example() {
        assert_eq!(
            parse_input("F10\nN3\nF7\nR90\nF11"),
            vec![
                (Action::MoveForward, 10),
                (Action::MoveNorth, 3),
                (Action::MoveForward, 7),
                (Action::TurnRight, 90),
                (Action::MoveForward, 11)
            ]
        );
    }

    #[test]
    fn test_example_actions() {
        let mut boat = Boat::new();
        boat.do_action(Action::MoveForward, 10);
        assert_eq!((boat.position_y, boat.position_x), (0, 10));
        boat.do_action(Action::MoveNorth, 3);
        assert_eq!((boat.position_y, boat.position_x), (3, 10));
        boat.do_action(Action::MoveForward, 7);
        assert_eq!((boat.position_y, boat.position_x), (3, 17));
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.orientation_y, boat.orientation_x), (-1, 0));
        boat.do_action(Action::MoveForward, 11);
        assert_eq!((boat.position_y, boat.position_x), (-8, 17));
    }

    #[test]
    fn test_solve_example() {
        assert_eq!(solve_part1(&parse_input("F10\nN3\nF7\nR90\nF11")), 25)
    }

    #[test]
    fn test_rotation() {
        let mut boat = Boat::new();
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, 1));
        boat.do_action(Action::TurnLeft, 180);
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, -1));
        boat.do_action(Action::TurnLeft, 180);
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, 1));
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.orientation_y, boat.orientation_x), (-1, 0));
        boat.do_action(Action::TurnLeft, 180);
        assert_eq!((boat.orientation_y, boat.orientation_x), (1, 0));
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, 1));
        boat.do_action(Action::TurnRight, 270);
        assert_eq!((boat.orientation_y, boat.orientation_x), (1, 0));
        boat.do_action(Action::TurnLeft, 90);
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, -1));
        boat.do_action(Action::TurnRight, 180);
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, 1));
        boat.do_action(Action::TurnRight, 360);
        assert_eq!((boat.orientation_y, boat.orientation_x), (0, 1));
    }
}
