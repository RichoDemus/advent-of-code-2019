pub fn six() {}

fn calculate_number_of_orbits(input: Vec<&str>) -> u32 {
    let mut bodies = vec![];

    for b in input {
        let split: Vec<&str> = b.split(")").collect();
        let name = split.get(1).expect("no name!").to_string();
        let parent = split.get(0).expect("no parent").to_string();
        bodies.push(Body { name, parent })
    }

    let mut orbits = 0;
    for body in bodies.clone() {
        orbits += calculate_indirect_orbits(&body, &bodies);
    }

    orbits
}

// calculate distance to COM?
fn calculate_indirect_orbits(body: &Body, bodies: &Vec<Body>) -> u32 {
    let mut orbits = 0;
    let mut current = body;
    loop {
        let parent = &current.parent;
        if parent == &"COM".to_string() {
            orbits += 1;
            break
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

#[derive(Debug, Clone)]
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
            "COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L",
        ];

        assert_eq!(calculate_number_of_orbits(input), 42);
    }

    #[test]
    fn test_distance() {
        println!("wat");
        let bodies = vec![
            Body { name: "B".to_string(), parent: "COM".to_string() },
            Body { name: "C".to_string(), parent: "B".to_string() },
            Body { name: "D".to_string(), parent: "C".to_string() },
        ];
        assert_eq!(calculate_indirect_orbits(&Body { name: "D".to_string(), parent: "C".to_string() }, &bodies), 3);
    }
}
