use std::cmp;
use std::slice::Iter;

use itertools::Itertools;


#[derive(Debug)]
pub(crate) struct Intputer {
    memory: Vec<i32>,
    instruction_pointer: usize,
    //which "int" of instructions we're processing
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl Intputer {
    pub(crate) fn with_input(intcode: &str, inputs: Vec<i32>) -> Intputer {
       Intputer::with_input_vectorized_code(str_to_vec(intcode), inputs)
    }

    fn with_input_vectorized_code(codes: Vec<i32>, inputs: Vec<i32>) -> Intputer {
        Intputer {
            memory: codes,
            instruction_pointer: 0,
            inputs,
            outputs: vec![],
        }
    }

    pub(crate) fn run(&mut self) -> Vec<i32> {
        while self.process() == false {}
        self.outputs.clone()
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

#[cfg(test)]
mod tests {
    use std::slice::Iter;

    use super::*;

    #[test]
    fn test_simple_input_output() {
        let inputs = vec![1337];
        let mut intputer = Intputer::with_input("3,0,4,0,99", inputs);
        let outputs = intputer.run();
        assert_eq!(outputs.len(), 1);
        let result = outputs.get(0).cloned().unwrap();
        assert_eq!(result, 1337);
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
        let mut intputer = Intputer::with_input(program, inputs);
        let outputs = intputer.run();
        println!("result: {:?}", outputs);
        assert_eq!(outputs, vec![220]);
    }
}
