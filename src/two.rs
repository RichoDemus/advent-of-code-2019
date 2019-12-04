use itertools::Itertools;

const ORIGINAL_INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0";
const MODIFIED_INPUT: &str = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0";

fn exec(intcode: &str) -> String {
    let codes: Vec<i32> = str_to_vec(intcode);
    let mut intputer = Intputer::from(codes);

    println!("Intputer:{:?}", intputer);
    intputer.get_memory()
}

#[derive(Debug)]
struct Intputer {
    memory: Vec<i32>
}

impl Intputer {
    fn from(codes: Vec<i32>) -> Intputer {
        Intputer { memory: codes }
    }

    fn get_memory(self) -> String {
        self.memory.iter()
            .cloned()
            .map(|val|val.to_string())
            .join(",")
    }
}

fn str_to_vec(str: &str) -> Vec<i32> {
    str.split(",")
        .map(|code| code.parse::<i32>().expect("failed to parse to int"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(exec("1,0,0,0,99").to_string(), "2,0,0,0,99");
        assert_eq!(exec("2,3,0,3,99").to_string(), "2,3,0,6,99");
        assert_eq!(exec("2,4,4,5,99,0").to_string(), "2,4,4,5,99,9801");
        assert_eq!(exec("1,1,1,4,99,5,6,0,99").to_string(), "30,1,1,4,2,5,6,0,99");

        println!("answer: {:?}", exec(MODIFIED_INPUT));
    }
}