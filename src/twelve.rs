use std::str::FromStr;

use crate::read_lines::read_lines;

#[derive(Eq, PartialEq, Debug)]
struct Body {
    y: i32,
    x: i32,
    z: i32,
    dy: i32,
    dx: i32,
    dz: i32,
}

impl Body {
    fn from(s: &str) -> Body {
        if s.contains("vel") {
            println!("s: {:?}", s);
            let mut iter = s.split("=");
            iter.next();
            iter.next();
            let x = iter.next().expect("iter next");
            let x = x.split(',').collect::<Vec<&str>>();
            let x = x.first().expect(" getting x");
            let x = x.trim();
            let x = i32::from_str(x).expect(format!("failed to parse x ({:?}) to i32", x).as_str());

            let y = iter.next().expect("iter next");
            let y = y.split(',').collect::<Vec<&str>>();
            let y = y.first().expect(" getting y");
            let y = y.trim();
            let y = i32::from_str(y).expect("failed to parse y to i32");

            let z: &str = iter.next().expect("iter next");
            let z = z.split('>').collect::<Vec<&str>>();
            let z = z.first().expect(" getting z");
            let z = z.trim();
            let z = i32::from_str(z).expect("failed to parse z to i32");

            iter.next();
            let dx: &str = iter.next().expect("iter next");
            let dx = dx.split(',').collect::<Vec<&str>>();
            let dx = dx.first().expect(" getting dx");
            let dx = dx.trim();
            let dx = i32::from_str(dx).expect("failed to parse dx to i32");

            let dy: &str = iter.next().expect("iter next");
            let dy = dy.split(',').collect::<Vec<&str>>();
            let dy = dy.first().expect(" getting dy");
            let dy = dy.trim();
            let dy = i32::from_str(dy).expect("failed to parse dy to i32");

            let dz: &str = iter.next().expect("iter next");
            let dz = dz.split('>').collect::<Vec<&str>>();
            let dz = dz.first().expect(" getting dz");
            let dz = dz.trim();
            let dz = i32::from_str(dz).expect("failed to parse dz to i32");

            Body { y, x, z, dy, dx, dz }
        } else {
            let split: Vec<&str> = s.split_ascii_whitespace().collect();
            let mut iter = s.split_ascii_whitespace();
            let x = iter.next().expect("x part not found");
            let x = x.split('=').collect::<Vec<&str>>();
            let x: &str = x.last().expect("couldn't get actual x value");
            let x = x.split(",").collect::<Vec<&str>>();
            let x = x.first().expect("couldn't remove ,");
            let x = i32::from_str(x).expect(format!("couldn't parse x ({:?}) to i32", x).as_str());

            let y = iter.next().expect("y part not found");
            let y = y.split('=').collect::<Vec<&str>>();
            let y: &str = y.last().expect("couldn't get actual y value");
            let y = y.split(",").collect::<Vec<&str>>();
            let y = y.first().expect("couldn't remove ,");
            let y = i32::from_str(y).expect(format!("couldn't parse y ({:?}) to i32", y).as_str());

            let z = iter.next().expect("z part not found");
            let z = z.split('=').collect::<Vec<&str>>();
            let z: &str = z.last().expect("couldn't get actual z value");
            let z = z.split(">").collect::<Vec<&str>>();
            let z = z.first().expect("couldn't remove ,");
            let z = i32::from_str(z).expect(format!("couldn't parse z ({:?}) to i32", z).as_str());

            Body { y, x, z, dy: 0, dx: 0, dz: 0 }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Space {
    bodies: Vec<Body>,
    tick: i32,
}

impl Space {
    fn parse(input: Vec<String>) -> Space {
        let bodies = input.into_iter()
            .map(|line| Body::from(&line))
            .collect::<Vec<Body>>();
        Space {
            bodies,
            tick: 0,
        }
    }

    fn parse_str(input: Vec<&str>) -> Space {
        let bodies = input.into_iter()
            .map(|line| Body::from(line))
            .collect::<Vec<Body>>();
        Space {
            bodies,
            tick: 0,
        }
    }

    fn tick(&mut self)  {
        self.tick += 1;
    }
}

pub fn twelve() {}

fn read_input() -> Vec<Body> {
    let raw = read_lines("twelve");
    raw.into_iter()
        .map(|line| Body::from(&line))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input() {
        let result = read_input();

        assert_eq!(result, vec![
            Body { x: 12, y: 0, z: -15, dx: 0, dy: 0, dz: 0 },
            Body { x: -8, y: -5, z: -10, dx: 0, dy: 0, dz: 0 },
            Body { x: 7, y: -17, z: 1, dx: 0, dy: 0, dz: 0 },
            Body { x: 2, y: -11, z: -6, dx: 0, dy: 0, dz: 0 },
        ]);
    }

    #[test]
    fn test_provided_example() {
        let input = vec![
            String::from("<x=-1, y=0, z=2>"),
            String::from("<x=2, y=-10, z=-7>"),
            String::from("<x=4, y=-8, z=8>"),
            String::from("<x=3, y=5, z=-1>"),
        ];

        let mut space = Space::parse(input);

        assert_eq!(space.tick, 0);
        assert_eq!(space, Space::parse_str(vec![
            "pos=<x=-1, y=  0, z= 2>, vel=<x= 0, y= 0, z= 0>",
            "pos=<x= 2, y=-10, z=-7>, vel=<x= 0, y= 0, z= 0>",
            "pos=<x= 4, y= -8, z= 8>, vel=<x= 0, y= 0, z= 0>",
            "pos=<x= 3, y=  5, z=-1>, vel=<x= 0, y= 0, z= 0>",
        ]));

        space.tick();

        assert_eq!(space.tick, 1);
        assert_eq!(space, Space::parse_str(vec![
            "pos=<x= 2, y=-1, z= 1>, vel=<x= 3, y=-1, z=-1>",
            "pos=<x= 3, y=-7, z=-4>, vel=<x= 1, y= 3, z= 3>",
            "pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>",
            "pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>",
        ]));
    }
}
