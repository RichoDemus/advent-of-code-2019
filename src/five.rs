use std::cmp;
use std::slice::Iter;

use itertools::Itertools;
use crate::read_lines::read_lines;

const ORIGINAL_INPUT: &str = "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0";
const MODIFIED_INPUT: &str = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,13,19,1,9,19,23,2,13,23,27,2,27,13,31,2,31,10,35,1,6,35,39,1,5,39,43,1,10,43,47,1,5,47,51,1,13,51,55,2,55,9,59,1,6,59,63,1,13,63,67,1,6,67,71,1,71,10,75,2,13,75,79,1,5,79,83,2,83,6,87,1,6,87,91,1,91,13,95,1,95,13,99,2,99,13,103,1,103,5,107,2,107,10,111,1,5,111,115,1,2,115,119,1,119,6,0,99,2,0,14,0";


pub fn five() {
    let mut input = read_lines("five");
    let result = exec_with_input(input.pop().unwrap().as_str(), vec![1]);
    println!("{:?}", result);
}
fn exec_with_input(intcode: &str, inputs: Vec<i32>) -> Vec<i32> {
    let codes: Vec<i32> = str_to_vec(intcode);
    let mut intputer = Intputer::with_input(codes, inputs);

    println!("Intputer:{:?}", intputer);
    while intputer.process() == false {}
    intputer.outputs
}

fn exec_with_params(noun: i32, verb: i32) -> i32 {
    let input = add_parameters(ORIGINAL_INPUT, noun.to_string().as_str(), verb.to_string().as_str());
    let result = exec(input.as_str());
    let split: Vec<&str> = result.split(",").collect();
    let first = split.first().expect("couldnt get program output");
    first.parse::<i32>().expect("can't parse str to int")
}

fn exec(intcode: &str) -> String {
    let codes: Vec<i32> = str_to_vec(intcode);
    let mut intputer = Intputer::from(codes);

    println!("Intputer:{:?}", intputer);
    while intputer.process() == false {}
    intputer.get_memory()
}

#[derive(Debug)]
struct Intputer {
    memory: Vec<i32>,
    instruction_pointer: usize,
    //which "int" of instructions we're processing
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl Intputer {
    fn from(codes: Vec<i32>) -> Intputer {
        Intputer {
            memory: codes,
            instruction_pointer: 0,
            inputs: vec![],
            outputs: vec![],
        }
    }

    fn with_input(codes: Vec<i32>, inputs: Vec<i32>) -> Intputer {
        Intputer {
            memory: codes,
            instruction_pointer: 0,
            inputs,
            outputs: vec![],
        }
    }

    fn get_memory(self) -> String {
        self.memory.iter()
            .cloned()
            .map(|val| val.to_string())
            .join(",")
    }

    // process the next operation, returns true if we're done
    fn process(&mut self) -> bool {
        let first_instruction = self.instruction_pointer;
        let last_instruction = cmp::min(first_instruction + 4, self.memory.len());
        println!("Reading instructions from {} to {}", first_instruction, last_instruction);
        let instructions = &self.memory[first_instruction..last_instruction];
        println!("instructions: {:?}", instructions);
        if let Some(99) = instructions.first() {
            return true;
        }

        let operation_and_modes = OperationAndModes::from(instructions.first().expect("couldn't get operation").clone());
        let opcode = operation_and_modes.operation;
        let first_position = instructions.get(1);
        let second_position = instructions.get(2);
        let result_position = instructions.get(3);

        println!("Processing code {}, pos {:?} and {:?}, result to {:?}", opcode, first_position, second_position, result_position);
        match opcode {
            1 => {
                let first_value = self.get_value(*first_position.expect("couldn't get first pointer") as usize, operation_and_modes.first_parameter_mode);
                let second_value = self.get_value(*second_position.expect("couldn't get second pointer") as usize, operation_and_modes.second_parameter_mode);
//                println!("adding {} and {} (pos {} and {}) and writing it to {}", first_value, second_value, first_position, second_position, result_position);
                let result = first_value + second_value;
                self.write(*result_position.expect("couldn't get result pointer") as usize, result);
                self.instruction_pointer += 4;
            }
            2 => {
                let first_value = self.get_value(*first_position.expect("couldn't get first pointer") as usize, operation_and_modes.first_parameter_mode);
                let second_value = self.get_value(*second_position.expect("couldn't get second pointer") as usize, operation_and_modes.second_parameter_mode);
//                println!("multiplying {} with {} (pos {} and {}) and writing it to {}", first_value, second_value, first_position, second_position, result_position);
                let result = first_value * second_value;
                self.write(*result_position.expect("couldn't get result pointer") as usize, result);
                self.instruction_pointer += 4;
            }
            3 => {
                let input = self.inputs.remove(0);
                self.write(*first_position.expect("couldn't get first pointer") as usize, input);
                self.instruction_pointer += 2;
            }
            4 => {
                let value = self.get_value(*first_position.expect("couldn't get first pointer") as usize, operation_and_modes.first_parameter_mode);
                self.outputs.push(value);
                self.instruction_pointer += 2;
            }
            _ => panic!("unexpected opcode: {}", opcode),
        }

        false
    }

    fn get_value(&self, index: usize, mode:i32) -> i32 {
        if mode == 0 {
            self.memory.get(index).expect("could not get value from memory").clone()
        } else {
            index as i32
        }
    }

    fn write(&mut self, position: usize, value: i32) {
        let old = std::mem::replace(&mut self.memory[position], value);
        println!("Replaced {} with {} at {}", old, value, position)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct OperationAndModes {
    operation: i32,
    first_parameter_mode: i32,
    second_parameter_mode: i32,
    third_parameter_mode: i32,
}


impl OperationAndModes {
    fn from(raw: i32) -> OperationAndModes {
        fn number_to_digits(number: u32) -> Vec<u32> {
            let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
            digits
        }

        let mut digits = number_to_digits(raw as u32);
        let operation = digits.pop().expect("no operation");
        digits.pop(); //throw away first digit of opcode
        let first_parameter_mode = digits.pop().unwrap_or(0);
        let second_parameter_mode = digits.pop().unwrap_or(0);
        let third_parameter_mode = digits.pop().unwrap_or(0);
        OperationAndModes {
            operation: operation as i32,
            first_parameter_mode: first_parameter_mode as i32,
            second_parameter_mode: second_parameter_mode as i32,
            third_parameter_mode: third_parameter_mode as i32,
        }
    }
}

fn str_to_vec(str: &str) -> Vec<i32> {
    str.split(",")
        .map(|code| code.parse::<i32>().expect("failed to parse to int"))
        .collect()
}

fn add_parameters(input: &str, noun: &str, verb: &str) -> String {
    let mut result = input.to_string();
    result.replace_range(2..5, format!("{},{}", noun, verb).as_str());
    result
}

#[cfg(test)]
mod tests {
    use std::slice::Iter;

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
    fn test_add_params() {
        assert_eq!(add_parameters("1,2,3,4,5,6,7,8,9", "11", "12"), "1,11,12,4,5,6,7,8,9");
    }

    #[test]
    fn test_simple_input_output() {
        let inputs = vec![1337];
        let outputs = exec_with_input("3,0,4,0,99", inputs);
        assert_eq!(outputs.len(), 1);
        let result = outputs.get(0).cloned().unwrap();
        assert_eq!(result, 1337);
    }

    #[test]
    fn test_parameter_modes() {
        let string = exec("1002,4,3,4,33");
        let memory = string.as_str();
        assert_eq!(memory, "1002,4,3,4,99")
    }

    #[test]
    fn test_parse_parameter_and_modes() {
        let expected = OperationAndModes {
            operation: 2,
            first_parameter_mode: 0,
            second_parameter_mode: 1,
            third_parameter_mode: 0,
        };
        assert_eq!(OperationAndModes::from(1002), expected);
    }

    #[test]
    fn i_try_to_write_a_program(){
        let inputs = vec![10,20];
        // does (input_one + 1) * input_two
        let program = "3,0,101,1,0,0,3,1,2,0,1,0,4,0,99";
        let outputs = exec_with_input(program, inputs);
        println!("result: {:?}", outputs);
        assert_eq!(outputs, vec![220]);
    }
}
