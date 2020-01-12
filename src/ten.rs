use std::cmp;
use std::cmp::Ordering;
use std::f64::consts::PI;

use float_cmp::approx_eq;
use math::round;

use crate::read_lines::read_lines;
use num::checked_pow;

pub fn ten() {
    let input = read_lines("ten");
    let best_spot = calc_best_spot_unparsed(input);
    println!("There are {} visible asteroids", best_spot.asteroids);
}

pub fn ten_part2() {
    println!("{:?}", solve_ten_part_2());
}

fn solve_ten_part_2() -> i32 {
    let input = read_lines("ten");
    let asteroids_removed = calc_asteroid_destruction_order(parse(input), None);
    let asteroid = asteroids_removed.get(199).expect("expected there to be 200 asteroids");
    asteroid.x * 100 + asteroid.y
}

#[derive(Debug)]
struct Spot {
    location: Point,
    asteroids: i32,
}

fn calc_best_spot_unparsed(input: Vec<String>) -> Spot {
    calc_best_spot(&parse(input))
}

fn calc_best_spot(field: &AsteroidField) -> Spot {
//    println!("Asteroids: {:?}", field.asteroids);

    let mut best_spot = None;

    for station_candidate in &field.asteroids {
//        println!("Checking {:?}", station_candidate);
        let observable_asteroids = calc_observable_asteroids2(&field, station_candidate);
//        println!("Can observe {} asteroids from {:?}", observable_asteroids, station_candidate);
        best_spot = match best_spot {
            None => Some(Spot { location: station_candidate.clone(), asteroids: observable_asteroids }),
            Some(current) => {
                if current.asteroids < observable_asteroids {
                    Some(Spot { location: station_candidate.clone(), asteroids: observable_asteroids })
                } else {
                    Some(current)
                }
            }
        }
    }
    best_spot.expect("should've found a spot!")
}

fn parse(input: Vec<String>) -> AsteroidField {
    let partial = parse_partial(input);
    let y_max = partial.len() as i32;
    let x_max = partial.get(0).expect("no lines on row 1").len() as i32;

    let parsed = parse_again(partial);
    AsteroidField {
        x_max,
        y_max,
        asteroids: parsed,
    }
}

fn parse_partial(input: Vec<String>) -> Vec<Vec<bool>> {
    input.into_iter().map(|row| {
        let row: Vec<bool> = row.chars()
            .map(|char| match char {
                '#' => true,
                _ => false,
            })
            .collect();
        row
    }).collect()
}

fn calc_observable_asteroids2(field: &AsteroidField, station: &Point) -> i32 {
    if !field.has_asteroid_at(station) {
        panic!("No asteroid at {:?}", station);
    }
    let mut obserable_asteroids = vec![];

    'outer: for asteroid in &field.asteroids {
        if asteroid == station {
            continue;
        }

        let x_direction = if station.x < asteroid.x {
            1
        } else {
            -1
        };

        let y_direction = if station.y < asteroid.y {
            1
        } else {
            -1
        };

        let dx = (station.x - asteroid.x).abs();
        let dy = (station.y - asteroid.y).abs();
        let greatest_common_divisor = ::num::integer::gcd(dx, dy);
        let dx = dx / greatest_common_divisor;
        let dy = dy / greatest_common_divisor;

        let mut x = station.x + dx * x_direction;
        let mut y = station.y + dy * y_direction;

//        println!("\tFor station {:?} to asteroid {:?}, dx: {}, xdir: {}, dy: {}, ydir: {}", station, asteroid, dx, x_direction, dy, y_direction);

        'inner: loop /*asteroid.x != x && asteroid.y != y*/ {
//            println!("\t\tx: {}, y: {}", x, y);
            if asteroid.x == x && asteroid.y == y {
                break 'inner;
            }
            let point = Point { x, y };
            if field.has_asteroid_at(&point) {
                // asteroid in the way
//                println!("\t\tAsteroid {:?} is between station {:?} and asteroid {:?}", point, station, asteroid);
                continue 'outer;
            }

            x += dx * x_direction;
            y += dy * y_direction;
            if y.abs() > 1000 { panic!("y Out of bounds") }
            if x.abs() > 1000 { panic!("x Out of bounds") }
        }
//        println!("\t\tNo asteroid between station {:?} and asteroid {:?}", station, asteroid);
        obserable_asteroids.push(asteroid);

//        println!("for station {:?} and asteroid {:?}, dx: {}, dy: {}", station, asteroid, dx, dy);


//        'inner: for other_asteroid in &asteroids {
//            if asteroid == other_asteroid {
//                continue 'inner;
//            }
//
//
//            obserable_asteroids.push(other_asteroid); //temp for compiler
//        }
    }
    obserable_asteroids.len() as i32
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Point {
    y: i32,
    x: i32,
}

struct AsteroidField {
    x_max: i32,
    y_max: i32,
    asteroids: Vec<Point>,
}

impl AsteroidField {
    fn has_asteroid_at(&self, point: &Point) -> bool {
        self.asteroids.iter().any(|asteroid| asteroid == point)
    }

    fn shoot_at(&mut self, point: &Point) -> ShootResult {
        match self.has_asteroid_at(point) {
            false => ShootResult::Miss,
            true => {
                self.asteroids = self.asteroids.iter()
                    .filter(|asteroid| asteroid != &point)
                    .cloned()
                    .collect();
                ShootResult::Hit
            }
        }
    }

    fn remove_asteroid_at(&mut self, point: &Point) {
        self.asteroids = self.asteroids.iter()
            .filter(|asteroid| asteroid != &point)
            .cloned()
            .collect();
    }
}

enum ShootResult {
    Hit,
    Miss,
}

//fn are_on_a_line(one: &Point, two: &Point, three: &Point) -> bool {
//    let left_hand = (two.y - one.y) * (three.x - one.x);
//    let right_hand = (three.y - one.y) * (two.x - one.x);
//    left_hand == right_hand
//}

fn parse_again(map: Vec<Vec<bool>>) -> Vec<Point> {
    let mut result = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, b) in row.iter().enumerate() {
            if *b {
                result.push(Point { x: x as i32, y: y as i32 });
            }
        }
    };

    result
}

struct Angle {
    dy: i32,
    dx: i32,
    laser_length: i32,
}

impl Angle {
    fn upwards(laser_length: i32) -> Angle {
        Angle { dy: -1, dx: 0, laser_length }
    }
    fn next_point_in_line(&self, x: i32, y: i32) -> (i32, i32) {
        (x + self.dx, y + self.dy)
    }
}

fn find_index_of_first_asteroid_with_non_negative_angle(asteroids: &Vec<(f64, Point)>) -> usize {
    for (i, (angle, _)) in asteroids.iter().enumerate() {
        match angle.partial_cmp(&0_f64) {
            None => panic!("Can't compare {} and {}", angle, 0_64),
            Some(ordering) => match ordering {
                Ordering::Less => { /* noop */ }
                Ordering::Equal => { return i; }
                Ordering::Greater => { return i; }
            },
        }
    };

    0 // if there are no non-negative, return the first, I think....
}

fn calc_asteroid_destruction_order(mut field: AsteroidField, station: Option<Point>) -> Vec<Point> {
    let station = station.unwrap_or_else(|| {
        let station = calc_best_spot(&field);
        let station = station.location;
        station
    });

    field.remove_asteroid_at(&station);

    let mut asteroids_by_angles = group_asteroids_by_angles(&field.asteroids, &station);


//    println!("Asteroids: {:?}", asteroids_by_angles);

    let (mut current_angle, _) = asteroids_by_angles.get(0).expect("no asteroids....");
    let mut first_index = 0;
    let mut last_index = 0;
    let mut removed_asteroids = vec![];
    while !field.asteroids.is_empty() { // todo use asteroids_by_angles instead
        let mut asteroids_for_current_angle = vec![];
        let (tmp_angle, _) = asteroids_by_angles.get(first_index).expect(format!("no asteroid at {}....", first_index).as_str());
        current_angle = *tmp_angle;
//        println!("\nAsteroids left: {:?}", field.asteroids);
//        println!("Angle is: {}", current_angle);
        'asteroid_line_scan: for (index, (angle, asteroid)) in asteroids_by_angles.iter().enumerate() {
            if index < first_index {
                continue;
            }
//            println!("\tChecking if {} equals {}", current_angle, *angle);
            if approx_eq!(f64, current_angle, *angle) {
//                println!("\t\teq");
                asteroids_for_current_angle.push(asteroid.clone());
                last_index = index;
            } else {
//                println!("\t\tnot eq");
//                println!("\t{} != {}", current_angle, *angle);
                break 'asteroid_line_scan;
            }
        }

//        println!("asteroids in field before blast {:?}", field.asteroids);
//        println!("Asteroids in line for angle {}: {:?}", current_angle, asteroids_for_current_angle);
        let asteroid_to_remove = get_closest_asteroid(asteroids_for_current_angle, &station);
//        let asteroid_to_remove = asteroids_for_current_angle.get(0).expect("no asteroid for angle");
        field.remove_asteroid_at(&asteroid_to_remove);
        asteroids_by_angles = asteroids_by_angles.into_iter()
            .filter(|(_, asteroid)| {
                asteroid != &asteroid_to_remove
            })
            .collect();
        removed_asteroids.push(asteroid_to_remove.clone().clone());
//        println!("asteroids in field after blast {:?}", field.asteroids);
        first_index = last_index; // we don't increment with one since we're removing one element
        if first_index == asteroids_by_angles.len() {
            first_index = 0;
        }
    }
    removed_asteroids

//    let max_laser_length = calculate_max_laser_length(&field, &station);
//    let mut angle = Angle::upwards(max_laser_length);
//
//
//    let mut destroyed_asteroids = vec![];
//    while !field.asteroids.is_empty() {
//        let mut x = station.x;
//        let mut y = station.y;
//        'shoot_angle: for i in 0..max_laser_length {
//            let (x, y) = angle.next_point_in_line(x, y);
//            let maybe_asteroid = Point { x, y };
//            match field.shoot_at(&maybe_asteroid) {
//                ShootResult::Hit => {
//                    destroyed_asteroids.push(maybe_asteroid);
//                    break 'shoot_angle;
//                }
//                ShootResult::Miss => {}
//            }
//        }
////        angle.next();
//    }

//    println!("-90: {:?}", -90_f64.to_radians().tan());
//
//    for i in 0..=900 {
//        let i = f64::from(i);
//        let i = i * 0.1_f64;
//        let tan = i.to_radians().tan();
//        let rounded = round::half_up(tan, 5);
//        let one_div = 1_f64 / tan;
//        let one_div_rounded = round::half_to_even(1_f64 / tan, 0);
//        println!("{}: {:?} ~ {} => {}", i, rounded, one_div, one_div_rounded);
//    }
}

fn get_closest_asteroid(mut asteroids: Vec<Point>, station: &Point) -> Point {
    let calc_distance = |asteroid: &Point| {
        let x = station.x - asteroid.x;
        let x = checked_pow(x,2).expect("failed to to pow");
        let y = station.y - asteroid.y;
        let y = checked_pow(y,2).expect("failed to to pow");

        let distance = ((x+y) as f64).sqrt();
        distance
    };

    asteroids.sort_by(|asteroid, other_asteroid| {
            calc_distance(asteroid).partial_cmp(&calc_distance(other_asteroid)).expect("couldn't order two distances")
    });

//    println!("sorted asteroids: {:?}", asteroids);
    asteroids.get(0).cloned().expect("no asteroids after sort")
}

fn calculate_max_laser_length(field: &AsteroidField, station: &Point) -> i32 {
    let x = station.x;
    let y = station.y;
    let max_y = cmp::max(y, field.y_max - y - 1);
    let max_x = cmp::max(x, field.x_max - x - 1);
    cmp::max(max_x, max_y)
}

fn group_asteroids_by_angles(asteroids: &Vec<Point>, station: &Point) -> Vec<(f64, Point)> {
    let mut result = vec![];
    for asteroid in asteroids {
        if asteroid == station {
            continue;
        }

        let dx = asteroid.x - station.x;
        let dy = station.y - asteroid.y; // these are flipped since 0,0 is top left and not bottom left

        let dx = f64::from(dx);
        let dy = f64::from(dy);

        let theta_degrees = (PI - dy.atan2(dx)).to_degrees() + 270_f64;
        result.push((theta_degrees, asteroid.clone()))
    }
    let mut result: Vec<(f64, Point)> = result.into_iter().map(|(angle, asteroid)| {
        match angle.partial_cmp(&360_f64) {
            None => panic!("Can't compare {} and {}", angle, &360_f64),
            Some(ordering) => match ordering {
                Ordering::Less => (angle, asteroid),
                Ordering::Equal => (angle - 360_f64, asteroid),
                Ordering::Greater => (angle - 360_f64, asteroid),
            },
        }
    }).collect();
    result.sort_by(|(left_angle, _), (right_angle, _)| left_angle.partial_cmp(right_angle).unwrap());
//    result.reverse();
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn first_test_case() {
        let input = vec![
            String::from(".#..#"),
            String::from("....."),
            String::from("#####"),
            String::from("....#"),
            String::from("...##"),
        ];

        let result = calc_best_spot_unparsed(input);
        println!("{:?}", result);
        assert_eq!(result.location.x, 3);
        assert_eq!(result.location.y, 4);
        assert_eq!(result.asteroids, 8);
    }

    #[test]
    fn my_test_case() {
        let input = vec![
            String::from("#...."),
            String::from("..#.."),
            String::from("#...#"),
        ];

        let result = calc_observable_asteroids2(&parse(input), &Point { x: 0, y: 0 });

        assert_eq!(result, 2);
    }

    #[test]
    fn test_reverse_direction_block() {
        let input = vec![
            String::from("#...."),
            String::from("..#.."),
            String::from("#...#"),
        ];

        let result = calc_observable_asteroids2(&parse(input), &Point { x: 4, y: 2 });

        assert_eq!(result, 2);
    }

    #[test]
    fn my_test_case2() {
        let input = vec![
            String::from("#...."),
            String::from("###.."),
            String::from("#...#"),
        ];

        let result = calc_observable_asteroids2(&parse(input), &Point { x: 2, y: 1 });

        assert_eq!(result, 4);
    }

    #[test]
    fn test_direction_asteroid_above() {
        let input = vec![
            String::from("#...."),
            String::from("....."),
            String::from("#...."),
        ];

        let result = calc_observable_asteroids2(&parse(input), &Point { x: 0, y: 2 });

        assert_eq!(result, 1);
    }

    #[test]
    fn test_direction_asteroid_to_left() {
        let input = vec![
            String::from("#...#"),
            String::from("....."),
            String::from("....."),
        ];

        let result = calc_observable_asteroids2(&parse(input), &Point { x: 4, y: 0 });

        assert_eq!(result, 1);
    }

    #[test]
    fn test_direction_asteroid_to_left_and_above() {
        let input = vec![
            String::from("#...."),
            String::from("....."),
            String::from("....#"),
        ];

        let result = calc_observable_asteroids2(&parse(input), &Point { x: 4, y: 2 });

        assert_eq!(result, 1);
    }

    #[test]
    fn test_has_asteroid_at() {
        let field = AsteroidField {
            x_max: 2,
            y_max: 2,
            asteroids: vec![Point { x: 1, y: 1 }],
        };
        assert_eq!(field.has_asteroid_at(&Point { x: 0, y: 0 }), false);
        assert_eq!(field.has_asteroid_at(&Point { x: 1, y: 1 }), true);
        assert_eq!(field.has_asteroid_at(&Point { x: 1, y: 0 }), false);
    }

    #[test]
    fn solve_first() {
        let input = read_lines("ten");
        let best_spot = calc_best_spot_unparsed(input);
        assert_eq!(best_spot.asteroids, 280);
    }

    #[test]
    fn part2_test_case() {
        let input = vec![
            String::from(".#..##.###...#######"),
            String::from("##.############..##."),
            String::from(".#.######.########.#"),
            String::from(".###.#######.####.#."),
            String::from("#####.##.#.##.###.##"),
            String::from("..#####..#.#########"),
            String::from("####################"),
            String::from("#.####....###.#.#.##"),
            String::from("##.#################"),
            String::from("#####.##.###..####.."),
            String::from("..######..##.#######"),
            String::from("####.##.####...##..#"),
            String::from(".#####..#.######.###"),
            String::from("##...#.##########..."),
            String::from("#.##########.#######"),
            String::from(".####.#.###.###.#.##"),
            String::from("....##.##.###..#####"),
            String::from(".#.#.###########.###"),
            String::from("#.#.#.#####.####.###"),
            String::from("###.##.####.##.#..##"),
        ];

        let asteroid_destruction_order = calc_asteroid_destruction_order(parse(input), None);

        let assert = |num: usize, x: i32, y: i32| {
            assert_eq!(asteroid_destruction_order.get(num - 1).unwrap(), &Point { x, y });
        };

        assert(1, 11, 12);
        assert(2, 12, 1);
        assert(3, 12, 2);
        assert(10, 12, 8);
        assert(20, 16, 0);
        assert(50, 16, 9);
        assert(100, 10, 16);
        assert(199, 9, 6);
        assert(200, 8, 2);
        assert(201, 10, 9);
        assert(299, 11, 1);
    }

    #[test]
    fn small_example_part2() {
        let input = vec![
            String::from("#.#.#"),
            String::from("###.#"),
            String::from("..#.."),
        ];

        let asteroid_destruction_order = calc_asteroid_destruction_order(parse(input), Some(Point { x: 2, y: 1 }));

        assert_eq!(asteroid_destruction_order, vec![
            Point { y: 0, x: 2 },
            Point { y: 0, x: 4 },
            Point { y: 1, x: 4 },
            Point { y: 2, x: 2 },
            Point { y: 1, x: 1 },
            Point { y: 0, x: 0 },
            Point { y: 1, x: 0 },
        ]);
    }

    #[test]
    fn test_calc_max_laser_length() {
        assert_eq!(calculate_max_laser_length(&AsteroidField { y_max: 10, x_max: 10, asteroids: vec![] }, &Point { x: 3, y: 4 }), 6);
        assert_eq!(calculate_max_laser_length(&AsteroidField { y_max: 10, x_max: 10, asteroids: vec![] }, &Point { x: 0, y: 4 }), 9);
        assert_eq!(calculate_max_laser_length(&AsteroidField { y_max: 10, x_max: 10, asteroids: vec![] }, &Point { x: 9, y: 4 }), 9);
    }

    #[test]
    fn test_atan_version() {
        let input = vec![
            String::from("#.#.."),
            String::from("###.#"),
            String::from("#.#.#"),
        ];

        let input = vec![
            String::from(".#..##.###...#######"),
            String::from("##.############..##."),
            String::from(".#.######.########.#"),
            String::from(".###.#######.####.#."),
            String::from("#####.##.#.##.###.##"),
            String::from("..#####..#.#########"),
            String::from("####################"),
            String::from("#.####....###.#.#.##"),
            String::from("##.#################"),
            String::from("#####.##.###..####.."),
            String::from("..######..##.#######"),
            String::from("####.##.####...##..#"),
            String::from(".#####..#.######.###"),
            String::from("##...#.##########..."),
            String::from("#.##########.#######"),
            String::from(".####.#.###.###.#.##"),
            String::from("....##.##.###..#####"),
            String::from(".#.#.###########.###"),
            String::from("#.#.#.#####.####.###"),
            String::from("###.##.####.##.#..##"),
        ];

        let mut angles_and_asteroids = group_asteroids_by_angles(&parse(input).asteroids, &Point { x: 7, y: 7 });
        angles_and_asteroids.sort_by(|(left_angle, _), (right_angle, _)| left_angle.partial_cmp(right_angle).unwrap());
        angles_and_asteroids.reverse();
        for (angle, point) in angles_and_asteroids {
            println!("y: {}, x: {} => {}", point.y, point.x, angle);
        }
    }

    #[test]
    fn assert_part2_solution() {
        assert_eq!(solve_ten_part_2(), 706  );
    }
}
