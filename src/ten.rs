use crate::read_lines::read_lines;

pub fn ten() {
    let input = read_lines("ten");
    let best_spot = calc_best_spot(input);
    println!("There are {} visible asteroids", best_spot.asteroids);
}

#[derive(Debug)]
struct Spot {
    location: Point,
    asteroids: i32,
}

fn calc_best_spot(input: Vec<String>) -> Spot {
    let field = parse(input);
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
//    let y_max = partial.len() as i32;
//    let x_max = partial.get(0).expect("no lines on row 1").len() as i32;

    let parsed = parse_again(partial);
    AsteroidField {
//        x_max,
//        y_max,
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
    x: i32,
    y: i32,
}

struct AsteroidField {
//    x_max: i32,
//    y_max: i32,
    asteroids: Vec<Point>,
}

impl AsteroidField {
    fn has_asteroid_at(&self, point: &Point) -> bool {
        self.asteroids.iter().any(|asteroid| asteroid == point)
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test_case() {
        let input = vec![
            String::from(".#..#"),
            String::from("....."),
            String::from("#####"),
            String::from("....#"),
            String::from("...##"),
        ];

        let result = calc_best_spot(input);
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
//            x_max: 2,
//            y_max: 2,
            asteroids: vec![Point { x: 1, y: 1 }],
        };
        assert_eq!(field.has_asteroid_at(&Point { x: 0, y: 0 }), false);
        assert_eq!(field.has_asteroid_at(&Point { x: 1, y: 1 }), true);
        assert_eq!(field.has_asteroid_at(&Point { x: 1, y: 0 }), false);
    }

    #[test]
    fn solve_first() {
        let input = read_lines("ten");
        let best_spot = calc_best_spot(input);
        assert_eq!(best_spot.asteroids, 280);
    }
}
