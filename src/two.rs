use itertools::Itertools;
use std::cmp;

const ORIGINAL_INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0";
const MODIFIED_INPUT: &str = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0";

pub fn bruteforce() {
    for noun in 1..100 {
        for verb in 1..100 {
            let result = exec_with_params(noun, verb);
            if result == 19690720 {
                println!("verb {} and nounc {} is right, for the answer {}", noun, verb, (100 * noun + verb));
                panic!();
            }
            print!(".");
        }
    }
}

fn exec_with_params(noun: i32, verb: i32) -> i32 {
    let input = add_parameters(ORIGINAL_INPUT, noun.to_string().as_str(), verb.to_string().as_str());
    let result = exec(input.as_str());
    let split:Vec<&str> = result.split(",").collect();
    let first = split.first().expect("couldnt get program output");
    first.parse::<i32>().expect("can't parse str to int")
}

fn exec(intcode: &str) -> String {
    let codes: Vec<usize> = str_to_vec(intcode);
    let mut intputer = Intputer::from(codes);

    println!("Intputer:{:?}", intputer);
    while intputer.process() == false {}
    intputer.get_memory()
}

#[derive(Debug)]
struct Intputer {
    memory: Vec<usize>,
    instruction_row: usize, //which "row" of instructions we're processing
}

impl Intputer {
    fn from(codes: Vec<usize>) -> Intputer {
        Intputer {
            memory: codes,
            instruction_row: 0,
        }
    }

    fn get_memory(self) -> String {
        self.memory.iter()
            .cloned()
            .map(|val| val.to_string())
            .join(",")
    }

    // process the next row, returns true if we're done
    fn process(&mut self) -> bool {
        let first_instruction = self.instruction_row * 4;
        let last_instruction = cmp::min(self.instruction_row * 4 + 4, self.memory.len());
        println!("Reading instructions from {} to {}", first_instruction, last_instruction);
        let instructions = &self.memory[first_instruction..last_instruction];
        println!("instructions: {:?}", instructions);
        self.instruction_row += 1;
        if let Some(99) = instructions.first() {
            return true;
        }

        let opcode = instructions.first().expect("couldn't get operation");
        let first_position = instructions.get(1).expect("couldn't get first pointer");
        let second_position = instructions.get(2).expect("couldn't get second pointer");
        let result_position = instructions.get(3).expect("couldn't get result pointer");

        println!("Processing code {}, pos {} and {}, result to {}", opcode, first_position, second_position, result_position);
        match opcode {
            1 => {
                let first_value = self.get_value(*first_position);
                let second_value = self.get_value(*second_position);
                println!("adding {} and {} (pos {} and {}) and writing it to {}", first_value, second_value, first_position, second_position, result_position);
                let result = first_value + second_value;
                self.write(*result_position, result)
            },
            2 => {
                let first_value = self.get_value(*first_position);
                let second_value = self.get_value(*second_position);
                println!("multiplying {} with {} (pos {} and {}) and writing it to {}", first_value, second_value, first_position, second_position, result_position);
                let result = first_value * second_value;
                self.write(*result_position, result)
            },
            _ => panic!("unexpected opcode: {}", opcode),
        }

        false
    }

    fn get_value(&self, index:usize) -> usize {
        self.memory.get(index).expect("could not get value from memory").clone()
    }

    fn write(&mut self, position:usize, value:usize) {
        let old = std::mem::replace(&mut self.memory[position], value);
        println!("Replaced {} with {} at {}", old, value, position)
    }
}

fn str_to_vec(str: &str) -> Vec<usize> {
    str.split(",")
        .map(|code| code.parse::<usize>().expect("failed to parse to int"))
        .collect()
}

fn add_parameters(input: &str, noun: &str, verb:&str) -> String {
    let mut result = input.to_string();
    result.replace_range(2..5, format!("{},{}", noun, verb).as_str());
    result
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
        assert_eq!(exec(MODIFIED_INPUT).to_string(), "3790689,12,2,2,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,60,1,9,19,63,2,13,23,315,2,27,13,1575,2,31,10,6300,1,6,35,6302,1,5,39,6303,1,10,43,6307,1,5,47,6308,1,13,51,6313,2,55,9,18939,1,6,59,18941,1,13,63,18946,1,6,67,18948,1,71,10,18952,2,13,75,94760,1,5,79,94761,2,83,6,189522,1,6,87,189524,1,91,13,189529,1,95,13,189534,2,99,13,947670,1,103,5,947671,2,107,10,3790684,1,5,111,3790685,1,2,115,3790687,1,119,6,0,99,2,0,14,0");

        println!("answer: {:?}", exec(MODIFIED_INPUT));
    }

    #[test]
    fn part2() {
        bruteforce();
    }

    #[test]
    fn test_add_params() {
        assert_eq!(add_parameters("1,2,3,4,5,6,7,8,9", "11", "12"), "1,11,12,4,5,6,7,8,9");
    }
}