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
pub struct BoatPart1 {
    angle: i32,
    orientation_y: i32,
    orientation_x: i32,
    position_y: i32,
    position_x: i32,
}

impl BoatPart1 {
    pub fn new() -> BoatPart1 {
        BoatPart1 {
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

struct BoatPart2 {
    position_y: i32,
    position_x: i32,
    waypoint_y: i32,
    waypoint_x: i32,
}

impl BoatPart2 {
    pub fn new() -> BoatPart2 {
        BoatPart2 {
            position_y: 0,
            position_x: 0,
            waypoint_y: 1,
            waypoint_x: 10,
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
        self.waypoint_y -= value
    }

    fn move_north(&mut self, value: i32) {
        self.waypoint_y += value
    }

    fn move_east(&mut self, value: i32) {
        self.waypoint_x += value
    }

    fn move_west(&mut self, value: i32) {
        self.waypoint_x -= value
    }

    fn turn_left(&mut self, value: i32) {
        let cos = (value as f64).to_radians().cos() as i32;
        let sin = (value as f64).to_radians().sin() as i32;
        let new_x = self.waypoint_x * cos - self.waypoint_y * sin;
        let new_y = self.waypoint_x * sin + self.waypoint_y * cos;
        self.waypoint_x = new_x;
        self.waypoint_y = new_y;
    }

    fn turn_right(&mut self, value: i32) {
        let cos = (-value as f64).to_radians().cos() as i32;
        let sin = (-value as f64).to_radians().sin() as i32;
        let new_x = self.waypoint_x * cos - self.waypoint_y * sin;
        let new_y = self.waypoint_x * sin + self.waypoint_y * cos;
        self.waypoint_x = new_x;
        self.waypoint_y = new_y;
    }

    fn move_forward(&mut self, value: i32) {
        self.position_x += self.waypoint_x * value;
        self.position_y += self.waypoint_y * value;
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
    let mut boat = BoatPart1::new();
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

pub fn solve_part2(input: &[(Action, i32)]) -> i32 {
    let mut boat = BoatPart2::new();
    input
        .iter()
        .for_each(|(action, value)| boat.do_action(*action, *value));
    boat.get_dist_from_start()
}

pub fn part2() {
    let mut file = File::open("input/2020/day12.txt").unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", solve_part2(&parse_input(&input)));
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
    fn test_part1_example_actions() {
        let mut boat = BoatPart1::new();
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
    fn test_part2_example_actions() {
        let mut boat = BoatPart2::new();
        boat.do_action(Action::MoveForward, 10);
        assert_eq!((boat.position_y, boat.position_x), (10, 100));
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (1, 10));
        boat.do_action(Action::MoveNorth, 3);
        assert_eq!((boat.position_y, boat.position_x), (10, 100));
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (4, 10));
        boat.do_action(Action::MoveForward, 7);
        assert_eq!((boat.position_y, boat.position_x), (38, 170));
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (4, 10));
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.position_y, boat.position_x), (38, 170));
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (-10, 4));
        boat.do_action(Action::MoveForward, 11);
        assert_eq!((boat.position_y, boat.position_x), (-72, 214));
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (-10, 4));
    }

    #[test]
    fn test_part1_solve_example() {
        assert_eq!(solve_part1(&parse_input("F10\nN3\nF7\nR90\nF11")), 25)
    }

    #[test]
    fn test_part1_rotation() {
        let mut boat = BoatPart1::new();
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

    #[test]
    fn test_part2_rotation() {
        let mut boat = BoatPart2::new();
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (1, 10));
        boat.do_action(Action::TurnLeft, 180);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (-1, -10));
        boat.do_action(Action::TurnRight, 180);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (1, 10));
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (-10, 1));
        boat.do_action(Action::TurnLeft, 180);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (10, -1));
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (1, 10));
        boat.do_action(Action::TurnRight, 270);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (10, -1));
        boat.do_action(Action::TurnLeft, 90);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (-1, -10));
        boat.do_action(Action::TurnRight, 180);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (1, 10));
        boat.do_action(Action::TurnRight, 360);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (1, 10));
        boat.waypoint_y = 4;
        boat.waypoint_x = 10;
        boat.do_action(Action::TurnRight, 90);
        assert_eq!((boat.waypoint_y, boat.waypoint_x), (-10, 4));
    }
}
