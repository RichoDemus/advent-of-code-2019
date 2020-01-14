use std::cmp;
use std::collections::HashMap;

use crate::eleven::Direction::{Down, Left, Right, Up};
use crate::intputer::Intputer;
use crate::intputer::Result::{AwaitingInput, Done, Output, Processing};
use crate::read_lines::read_lines;

pub fn eleven() {
    let answer = solve_eleven_part1();
    println!("The answer is {}", answer);

    solve_eleven_part2();
}

fn solve_eleven_part1() -> i32 {
    let input = &*get_program();
    let mut intputer = Intputer::new(input);

    let mut board = Board::new();
    loop {
        let color_under_robot = board.get_color_under_robot();
        intputer.input(color_under_robot as i64);
        let new_color = match intputer.run() {
            Output(color) => color as i32,
            Done => break,
            AwaitingInput => panic!("AwaitingInput not expected, expected a color"),
            Processing => panic!("Processing not expected, expected a color"),
        };
        let new_direction = match intputer.run() {
            Output(direction) => direction as i32,
            Done => break,
            AwaitingInput => panic!("AwaitingInput not expected, expected a direction"),
            Processing => panic!("Processing not expected, expected a direction"),
        };
//        println!("Robot: {},{} facing {:?} standing on {}, painting {} and moving {}",
//                 board.robot.position.y,
//                 board.robot.position.x,
//                 board.robot.direction,
//                 color_under_robot,
//                 new_color,
//                 new_direction
//        );

        board.paint_under_robot(new_color);
        board.turn_robot(new_direction);
//        println!("\tRobot now on: {},{} facing {:?}",
//                 board.robot.position.y,
//                 board.robot.position.x,
//                 board.robot.direction,
//        );
//        println!("panels: {:?}", board.painted_panels);
    }

    board.painted_panels.len() as i32
}

fn solve_eleven_part2() {
    let input = &*get_program();
    let mut intputer = Intputer::new(input);

    let mut board = Board::new();
    board.painted_panels.insert(Panel { position: Point { y: 100, x: 100 } }, 1);
    loop {
        let color_under_robot = board.get_color_under_robot();
        intputer.input(color_under_robot as i64);
        let new_color = match intputer.run() {
            Output(color) => color as i32,
            Done => break,
            AwaitingInput => panic!("AwaitingInput not expected, expected a color"),
            Processing => panic!("Processing not expected, expected a color"),
        };
        let new_direction = match intputer.run() {
            Output(direction) => direction as i32,
            Done => break,
            AwaitingInput => panic!("AwaitingInput not expected, expected a direction"),
            Processing => panic!("Processing not expected, expected a direction"),
        };

        board.paint_under_robot(new_color);
        board.turn_robot(new_direction);
    }

    println!("{:?}", board.painted_panels);
    let some_panel = board.painted_panels.keys().last().expect("no painted panels...");
    let mut x_min = some_panel.position.x;
    let mut x_max = some_panel.position.x;
    let mut y_min = some_panel.position.y;
    let mut y_max = some_panel.position.y;
    for panel in board.painted_panels.keys() {
        let x = panel.position.x;
        let y = panel.position.y;

        x_min = cmp::min(x_min, x);
        x_max = cmp::max(x_max, x);
        y_min = cmp::min(y_min, y);
        y_max = cmp::max(y_max, y);
    }

    println!("boundaries: y: {}-{}, x: {}-{}", y_min, y_max, x_min, x_max);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let tile = board.painted_panels.get(&Panel { position: Point { y, x } }).cloned().unwrap_or(0);
            let tile = if tile == 1 { "#" } else { " " };
            print!("{} ", tile);
        }
        println!()
    }
    // ABCLFUHJ
}

fn get_program() -> Box<str> {
    let input = read_lines("eleven");
    let input = input.get(0).expect("no lines in eleven.txt");
    Box::from(input.clone())
}

struct Board {
    painted_panels: HashMap<Panel, i32>,
    robot: Robot,
}

impl Board {
    fn new() -> Board {
        Board {
            painted_panels: HashMap::new(),
            robot: Robot {
                position: Point {
                    y: 100,
                    x: 100,
                },
                direction: Direction::Up,
            },
        }
    }

    fn get_color(&self, y: i32, x: i32) -> i32 {
//        println!("\tget color under robot: {}, {}: {:?}", y, x, self.painted_panels.get(&Panel { position: Point { y, x } }).cloned());
        self.painted_panels.get(&Panel { position: Point { y, x } }).cloned().unwrap_or(0)
//        if self.painted_panels.iter().any(|panel| panel.position.x == x && panel.position.y == y) {
//            1
//        } else {
//            0
//        }
    }

    fn set_color(&mut self, y: i32, x: i32, color: i32) {
        self.painted_panels.insert(Panel { position: Point { y, x } }, color);
    }

    fn get_color_under_robot(&self) -> i32 {
        self.get_color(self.robot.position.y, self.robot.position.x)
    }

    fn paint_under_robot(&mut self, color: i32) {
        self.set_color(self.robot.position.y, self.robot.position.x, color)
    }

    fn turn_robot(&mut self, direction: i32) {
        self.robot.turn(direction);
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Panel {
    position: Point,
}

#[derive(Debug, Eq, PartialEq)]
struct Robot {
    position: Point,
    direction: Direction,
}

impl Robot {
    fn from(y: i32, x: i32, direction: Direction) -> Robot {
        Robot { position: Point { y, x }, direction }
    }

    fn turn(&mut self, turn: i32) {
        if turn != 0 && turn != 1 {
            panic!("Turn can only be 0 or 1");
        }
        fn calc_new_direction(current_direction: &Direction, turn: i32) -> Direction {
            match current_direction {
                Up => if turn == 0 { Left } else { Right },
                Down => if turn == 0 { Right } else { Left },
                Left => if turn == 0 { Down } else { Up },
                Right => if turn == 0 { Up } else { Down },
            }
        }
        self.direction = calc_new_direction(&self.direction, turn);

        self.position = self.position.travel(&self.direction);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    y: i32,
    x: i32,
}

impl Point {
    fn travel(&self, direction: &Direction) -> Point {
        let y = self.y;
        let x = self.x;
        match direction {
            Up => Point::from(y + 1, x),
            Down => Point::from(y - 1, x),
            Left => Point::from(y, x - 1),
            Right => Point::from(y, x + 1),
        }
    }

    fn from(y: i32, x: i32) -> Point {
        if y < 0 { panic!("Can't create point with y < 0") }
        if x < 0 { panic!("Can't create point with x < 0") }
        Point { y, x }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn robot_drive_around() {
        let mut robot = Robot { position: Point { x: 10, y: 10 }, direction: Up };
        assert_eq!(robot, Robot::from(10, 10, Up));

        robot.turn(0);
        assert_eq!(robot, Robot::from(10, 9, Left));

        robot.turn(0);
        assert_eq!(robot, Robot::from(9, 9, Down));

        robot.turn(0);
        assert_eq!(robot, Robot::from(9, 10, Right));

        robot.turn(0);
        assert_eq!(robot, Robot::from(10, 10, Up));

        robot.turn(1);
        assert_eq!(robot, Robot::from(10, 11, Right));

        robot.turn(1);
        assert_eq!(robot, Robot::from(9, 11, Down));

        robot.turn(1);
        assert_eq!(robot, Robot::from(9, 10, Left));

        robot.turn(1);
        assert_eq!(robot, Robot::from(10, 10, Up));
    }

    #[test]
    #[should_panic]
    fn should_be_able_to_move_oob_left() {
        let mut robot = Robot::from(0, 0, Up);
        robot.turn(0);
    }

    #[test]
    #[should_panic]
    fn should_be_able_to_move_oob_down() {
        let mut robot = Robot::from(0, 0, Right);
        robot.turn(1);
    }

    #[test]
    fn test_paint_under_robot() {
        let mut board = Board::new();
        assert_eq!(board.get_color_under_robot(), 0);
        board.paint_under_robot(1);
        assert_eq!(board.get_color_under_robot(), 1);
    }

    #[test]
    fn actually_solve_eleven_part1() {
        let answer = solve_eleven_part1();
        assert_eq!(answer, 1732);
    }

    #[test]
    fn actually_solve_eleven_part2() {
        solve_eleven_part2();
    }
}
