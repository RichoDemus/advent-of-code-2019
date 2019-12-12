use crate::read_lines::read_lines;
use itertools::Itertools;

pub fn six() {
    let input = read_lines("six");
    let orbits = calculate_number_of_orbits(input.clone());
    println!("there are {} orbits", orbits);
    let jumps = minimum_orbital_transfers(input);
    println!("there are {} orbital transfers from YOU TO SAN", jumps);
}

fn calculate_number_of_orbits(input: Vec<String>) -> u32 {
    let bodies = parse_bodies(input);

    let mut orbits = 0;
    for body in bodies.clone() {
        orbits += calculate_indirect_orbits(&body, &bodies);
    }

    orbits
}

fn parse_bodies(input: Vec<String>) -> Vec<Body> {
    let mut bodies = vec![];

    for b in input {
        let split: Vec<&str> = b.split(")").collect();
        let name = split.get(1).expect("no name!").to_string();
        let parent = split.get(0).expect("no parent").to_string();
        bodies.push(Body { name, parent })
    }
    bodies
}

// calculate distance to COM?
fn calculate_indirect_orbits(body: &Body, bodies: &Vec<Body>) -> u32 {
    let mut orbits = 0;
    let mut current = body;
    loop {
        let parent = &current.parent;
        if parent == &"COM".to_string() {
            orbits += 1;
            break;
        }
        let matches: Vec<&Body> = bodies.iter().filter(|body| body.name == parent.clone()).collect();
        if matches.len() != 1 {
            panic!("not exactly 1 parent");
        }
        let parent = matches.get(0).unwrap();
        orbits += 1;
        current = parent.clone();
    }

    orbits
}

fn minimum_orbital_transfers(input: Vec<String>) -> i32 {
    let bodies = parse_bodies(input);

    let you = bodies.iter().find(|e| e.name == "YOU").expect("couldn't find YOU");
    let san = bodies.iter().find(|e| e.name == "SAN").expect("couldn't find SAN");

    let start = get_body_by_name(&bodies, &you.parent);
    let end = get_body_by_name(&bodies, &san.parent);

    println!("Find path between {:?} and {:?}", start, end);

    let path = get_path_rec(&bodies, &vec![start], &end);
    path.len() as i32 -1 //subtract one to get number of trips
}

fn get_path_rec(bodies: &Vec<Body>, path: &Vec<Body>, end: &Body) -> Vec<Body> {
//    println!("path: {:?}", path);
    if path.last().expect("path cant beempty") == end {
        println!("Done: {:?}", pretty_pring_path(path));
        return path.clone()
    }

    let paths = get_surrounding_bodies(bodies, path.last().expect("path is empty"));
    println!("\t orbiting bodies of {}: {:?}", path.last().unwrap().name, paths);
    let paths:Vec<Body> = paths.into_iter().filter(|b| !path.contains(b)).collect();

    for p in paths {
        let mut new_path = path.clone();
        new_path.push(p);
        let result = get_path_rec(bodies, &new_path, end);
        if !result.is_empty() {
            return result;
        }
    }

    println!("path {:?} is wrong, abandoning", pretty_pring_path(path));
    vec![]
}

fn pretty_pring_path(path: &Vec<Body>) -> String {
    path.iter().map(|p|p.name.clone()).join("->")
}

fn get_surrounding_bodies(bodies: &Vec<Body>, body: &Body) -> Vec<Body> {
    let mut result= vec![];

    let parent = bodies.iter().find(|e|e.name == body.parent);

    let mut orbiters:Vec<Body> = bodies.iter()
        .filter(|e|e.parent == body.name)
        .cloned()
        .collect();

    if parent.is_some() {
        result.push(parent.unwrap().clone());
    }
    result.append(&mut orbiters);
    result
}

fn get_body_by_name(bodies: &Vec<Body>, name: &String) -> Body {
    bodies.iter()
        .find(|e|&e.name == name)
        .cloned()
        .expect("couldn't find body")
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Body {
    name: String,
    parent: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let input = vec![
            "COM)B".to_string(),
            "B)C".to_string(),
            "C)D".to_string(),
            "D)E".to_string(),
            "E)F".to_string(),
            "B)G".to_string(),
            "G)H".to_string(),
            "D)I".to_string(),
            "E)J".to_string(),
            "J)K".to_string(),
            "K)L".to_string(),
        ];

        assert_eq!(calculate_number_of_orbits(input), 42);
    }

    #[test]
    fn test_distance() {
        let bodies = vec![
            Body { name: "B".to_string(), parent: "COM".to_string() },
            Body { name: "C".to_string(), parent: "B".to_string() },
            Body { name: "D".to_string(), parent: "C".to_string() },
        ];
        assert_eq!(calculate_indirect_orbits(&Body { name: "D".to_string(), parent: "C".to_string() }, &bodies), 3);
    }

    #[test]
    fn test_orbital_transfers() {
        let input = vec![
            "COM)B".to_string(),
            "B)C".to_string(),
            "C)D".to_string(),
            "D)E".to_string(),
            "E)F".to_string(),
            "B)G".to_string(),
            "G)H".to_string(),
            "D)I".to_string(),
            "E)J".to_string(),
            "J)K".to_string(),
            "K)L".to_string(),
            "K)YOU".to_string(),
            "I)SAN".to_string(),
        ];

        assert_eq!(minimum_orbital_transfers(input), 4);
    }
}
