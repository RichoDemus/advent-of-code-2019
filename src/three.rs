use crate::three::Direction::{Down, Left, Right, Up};

pub fn three() {}

fn manhattan_distance(first: &str, second: &str) -> i32 {
    -1
}

#[derive(Debug, Eq, PartialEq)]
struct Point { x: i32, y: i32 }

// I need to do the same thing here as in calculte path, should probably only do it in one place or something...

impl Point {
    fn next(&self, direction: &Direction) -> Vec<Point> {
        match direction {
            Up(steps) => {
                let mut points = vec![];
                for step in 0..*steps {
                    points.push(Point { x: self.x, y: self.y + 1 })
                }
                points
            }
            Down(steps) => {
                let mut points = vec![];
                for step in 0..*steps {
                    points.push(Point { x: self.x, y: self.y - 1 })
                }
                points
            }
            Left(steps) => {
                let mut points = vec![];
                for step in 0..*steps {
                    points.push(Point { x: self.x - 1, y: self.y })
                }
                points
            }
            Right(steps) => {
                let mut points = vec![];
                for step in 0..*steps {
                    points.push(Point { x: self.x + 1, y: self.y })
                }
                points
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    Down(i32),
    Up(i32),
    Left(i32),
    Right(i32),
}

fn calculate_path(instructions: &str) -> Vec<Point> {
    let instructions = split_input(instructions);
    let mut path: Vec<Point> = vec![];

    let mut location = Point { x: 0, y: 0 };

    for direction in instructions {
        let last = path.last();
        let last = last.unwrap_or(&Point { x: 0, y: 0 });
        let mut new_points = last.next(&direction);
        println!("Direction {:?} resulted in {:?}", direction, new_points);
        path.append(&mut new_points);
    }

    path
}

fn split_input(path: &str) -> Vec<Direction> {
    fn parse(str: &str) -> Direction {
        let (direction, length) = str.split_at(1);
        let length = length.parse().expect("Unable to parse length to i32");
        match direction {
            "U" => Up(length),
            "D" => Down(length),
            "L" => Left(length),
            "R" => Right(length),
            _ => panic!("Unknown direction {}", direction)
        }
    }

    let split: Vec<&str> = path.split(",").collect();
    split.into_iter().map(|direction| parse(direction)).collect()
}

#[cfg(test)]
mod tests {
    use crate::read_lines::read_lines;
    use crate::three::Direction::{Down, Right};

    use super::*;

    #[test]
    fn test_split_input() {
        assert_eq!(split_input("R75,D30,R83"), vec![Right(75), Down(30), Right(83)])
    }

    #[test]
    fn test_calculate_path() {
        let path = calculate_path("U1,R2,D2,L1,R1");
        assert_eq!(path, vec![
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: -1 },
            Point { x: 1, y: -1 },
            Point { x: 2, y: -1 },
        ]);
    }

    #[test]
    fn test() {
        assert_eq!(manhattan_distance("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(manhattan_distance("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);

        let input = read_lines("three");
        let distance = manhattan_distance(
            input.get(0).expect("couldn't get first input line"),
            input.get(1).expect("couldn't get second input line"),
        );
        println!("answer: {}", distance);
    }
}
