use std::cmp;

use crate::intputer::Result::{AwaitingInput, Done, Output, Processing};

#[derive(Debug)]
pub(crate) enum Result {
    Done,
    Output(i64),
    AwaitingInput,
    Processing, //internal
}

#[derive(Debug)]
pub(crate) struct Intputer {
    memory: Vec<i64>,
    instruction_pointer: usize,
    relative_base: usize,
    //which "int" of instructions we're processing
    inputs: Vec<i64>,
    outputs: Vec<i64>,
}

impl Intputer {
    pub(crate) fn new(intcode: &str) -> Intputer {
        Intputer {
            memory: str_to_vec(intcode),
            instruction_pointer: 0,
            relative_base: 0,
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub(crate) fn run_with_input_single_output(intcode: &str, input: i64) -> Option<i64> {
        Intputer::run_with_input(intcode, input).get(0).cloned()
    }

    pub(crate) fn run_no_input_single_output(intcode: &str) -> Option<i64> {
        Intputer::run_no_input(intcode).get(0).cloned()
    }

    pub(crate) fn run_with_input(intcode: &str, input: i64) -> Vec<i64> {
        let mut intputer = Intputer::new(intcode);
        intputer.input(input);
        let mut result = vec![];
        loop {
            match intputer.run() {
                Done => break,
                Output(output) => result.push(output),
                AwaitingInput => {}
                Processing => {}
            }
        };
        result
    }

    pub(crate) fn run_no_input(intcode: &str) -> Vec<i64> {
        let mut intputer = Intputer::new(intcode);
        let mut result = vec![];
        loop {
            match intputer.run() {
                Done => break,
                Output(output) => result.push(output),
                AwaitingInput => {}
                Processing => {}
            }
        };
        result
    }

    pub(crate) fn input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub(crate) fn run(&mut self) -> Result {
        loop {
            let res = self.process();
            println!("-\tResult: {:?}", res);
            if let Processing = res {
                continue;
            }
            return res;
        }
    }

    // process the next operation
    fn process(&mut self) -> Result {
        let first_instruction = self.instruction_pointer;
        let last_instruction = cmp::min(first_instruction + 4, self.memory.len());
        println!("Reading instructions from {} to {}", first_instruction, last_instruction);
        let instructions = &self.memory[first_instruction..last_instruction];
        println!("instructions: {:?}", instructions);
        if let Some(99) = instructions.first() {
            return Done;
        }

        let operation_and_modes = OperationAndModes::from(instructions.first().expect("couldn't get operation").clone());
        let opcode = operation_and_modes.operation;
        let first_position = instructions.get(1);
        let second_position = instructions.get(2);
        let third_position = instructions.get(3);

        println!("Processing code {}, pos {:?} and {:?}, result to {:?} with relative base {}", opcode, first_position, second_position, third_position, self.relative_base);
        let result: Result = match opcode {
            1 => {
                // sum pos1 and pos2, write to pos3
                let first_value = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                let second_value = self.get_value(*second_position.expect("couldn't get second pointer"), operation_and_modes.second_parameter_mode);
                println!("adding {} and {} (pos {:?} and {:?}) and writing it to {:?}", first_value, second_value, first_position, second_position, third_position);
                let result = first_value + second_value;
                let position = *third_position.expect("couldn't get result pointer");
                self.write(position, result, operation_and_modes.third_parameter_mode);
                self.instruction_pointer += 4;
                Processing
            }
            2 => {
                // multiply pos1 and pos2, write to pos3
                let first_value = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                let second_value = self.get_value(*second_position.expect("couldn't get second pointer"), operation_and_modes.second_parameter_mode);
//                println!("multiplying {} with {} (pos {} and {}) and writing it to {}", first_value, second_value, first_position, second_position, result_position);
                let result = first_value * second_value;
                let position = *third_position.expect("couldn't get result pointer");
                self.write(position, result, operation_and_modes.third_parameter_mode);
                self.instruction_pointer += 4;
                Processing
            }
            3 => {
                // write input to pos1
                if self.inputs.is_empty() {
                    AwaitingInput
                } else {
                    let input = self.inputs.remove(0);
                    let position = *first_position.expect("couldn't get first pointer");
                    self.write(position, input, operation_and_modes.first_parameter_mode);
                    self.instruction_pointer += 2;
                    Processing
                }
            }
            4 => {
                // output pos1
                let value = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                // self.outputs.push(value);
                self.instruction_pointer += 2;
                Output(value)
            }
            5 => {
                // jump-if-true: if the first parameter is non-zero,
                //it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                let value_to_check = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                if value_to_check != 0 {
                    let new_instruction_pointer = self.get_value(*second_position.expect("couldn't get second pointer"), operation_and_modes.second_parameter_mode);
                    self.instruction_pointer = new_instruction_pointer as usize;
                } else {
                    self.instruction_pointer += 3;
                }
                Processing
            }
            6 => {
                // jump-if-false: if the first parameter is zero,
                //it sets the instruction pointer to the value from the second parameter. Otherwise, it does nothing.
                let value_to_check = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                if value_to_check == 0 {
                    let new_instruction_pointer = self.get_value(*second_position.expect("couldn't get second pointer"), operation_and_modes.second_parameter_mode);
                    self.instruction_pointer = new_instruction_pointer as usize;
                } else {
                    self.instruction_pointer += 3;
                }
                Processing
            }
            7 => {
                // less than: if the first parameter is less than the second parameter,
                //it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let first_value = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                let second_value = self.get_value(*second_position.expect("couldn't get second pointer"), operation_and_modes.second_parameter_mode);
                if first_value < second_value {
                    let position = *third_position.expect("couldn't get result pointer");
                    self.write(position, 1, operation_and_modes.third_parameter_mode);
                } else {
                    let position = *third_position.expect("couldn't get result pointer");
                    self.write(position, 0, operation_and_modes.third_parameter_mode);
                }
                self.instruction_pointer += 4;
                Processing
            }
            8 => {
                // equals: if the first parameter is equal to the second parameter,
                //it stores 1 in the position given by the third parameter. Otherwise, it stores 0.
                let first_value = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                let second_value = self.get_value(*second_position.expect("couldn't get second pointer"), operation_and_modes.second_parameter_mode);
                if first_value == second_value {
                    let position = *third_position.expect("couldn't get result pointer");
                    self.write(position, 1, operation_and_modes.third_parameter_mode);
                } else {
                    let position = *third_position.expect("couldn't get result pointer");
                    self.write(position, 0, operation_and_modes.third_parameter_mode);
                }
                self.instruction_pointer += 4;
                Processing
            }
            9 => {
                // adjusts the relative base
                let relative_base_adjust = self.get_value(*first_position.expect("couldn't get first pointer"), operation_and_modes.first_parameter_mode);
                let old_base = self.relative_base;
                self.relative_base += relative_base_adjust as usize;
                println!("\tAdjusting relative base: {} + {} => {}", old_base, relative_base_adjust, self.relative_base);
                self.instruction_pointer += 2;
                Processing
            }
            _ => panic!("unexpected opcode: {}", opcode),
        };

        result
    }

    fn get_value(&self, index: i64, mode: i64) -> i64 {
        match mode {
            0 => {
                let index = index as usize;
                let value = self.memory.get(index).cloned().unwrap_or(0);
                println!("\tread {} from memory slot {}", value, index);
                value
            },
            1 => {
                let value = index as i64;
                println!("\tread constant {}", index);
                value
            },
            2 => {
//                let i = self.relative_base.checked_add(index).expect(format!("can't add {} + {}", self.relative_base, index).as_str());
                let i = (self.relative_base as i64 + index) as usize;
                let value = self.memory.get(i).cloned().unwrap_or(0);
                println!("\tread {} from relative memory slot {} + {}", value, self.relative_base, index);
                value
            },
            _ => panic!(format!("Unknown mode {}", mode)),
        }
    }

    fn write(&mut self, position: i64, value: i64, mode: i64) {
        match mode {
            0 => {
                let position = position as usize;
                if self.memory.len() <= position {
                    let old_len = self.memory.len();
                    let new_size = position.checked_add(1).expect(format!("can't add {} + {}", position, 1).as_str());
                    self.memory.resize(position + 1, 0);
                    println!("\t\tResized memory from {} to {}", old_len, self.memory.len());
                }
                let old = std::mem::replace(&mut self.memory[position], value);
                println!("Replaced {} with {} at {}", old, value, position)
            },
            1 => panic!("mode 1 is not supported for writing"),
            2 => {
                let position: usize = (self.relative_base as i64 + position) as usize;
                if self.memory.len() <= position {
                    let old_len = self.memory.len();
                    let new_size = position.checked_add(1).expect(format!("can't add {} + {}", position, 1).as_str());
                    self.memory.resize(position + 1, 0);
                    println!("\t\tResized memory from {} to {}", old_len, self.memory.len());
                }
                let old = std::mem::replace(&mut self.memory[position], value);
                println!("Replaced {} with {} at {}", old, value, position)
            },
            _ => todo!("mode {} not implemented for writing", mode),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct OperationAndModes {
    operation: i64,
    first_parameter_mode: i64,
    second_parameter_mode: i64,
    third_parameter_mode: i64,
}

impl OperationAndModes {
    fn from(raw: i64) -> OperationAndModes {
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
            operation: operation as i64,
            first_parameter_mode: first_parameter_mode as i64,
            second_parameter_mode: second_parameter_mode as i64,
            third_parameter_mode: third_parameter_mode as i64,
        }
    }
}

fn str_to_vec(str: &str) -> Vec<i64> {
    str.split(",")
        .map(|code| code.parse::<i64>().expect("failed to parse to int"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_input_output() {
        let mut intputer = Intputer::new("3,0,4,0,99");
        intputer.input(1337);
        let output = match intputer.run() {
            Output(out) => out,
            _ => panic!("wrong status"),
        };
        assert_eq!(output, 1337);
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
    fn i_try_to_write_a_program() {
        // does (input_one + 1) * input_two
        let program = "3,0,101,1,0,0,3,1,2,0,1,0,4,0,99";
        let mut intputer = Intputer::new(program);
        intputer.input(10);
        intputer.input(20);
        let output = match intputer.run() {
            Output(out) => out,
            _ => panic!("wrong status"),
        };
        assert_eq!(output, 220);
    }

    #[test]
    fn new_input_output() {
        // reads input, adds 1, outputs it, reads input, adds 1, outputs it, outputs 1337, ends
        let intcode = "3,0,1001,0,1,0,4,0,3,0,1001,0,1,0,4,0,104,1337,99";
//        let mut intputer = Intputer::with_input(intcode, vec![1,2]);
        let mut intputer = Intputer::new(intcode);
        if let Result::AwaitingInput = intputer.run() {} else { panic!("wrong result") };
        intputer.input(1);
        let output = if let Result::Output(out) = intputer.run() {
            out
        } else { panic!("wrong result") };
        assert_eq!(output, 2);

        if let Result::AwaitingInput = intputer.run() {} else { panic!("wrong result") };
        intputer.input(output);

        let output = if let Result::Output(out) = intputer.run() {
            out
        } else { panic!("wrong result") };
        assert_eq!(output, 3);

        let output = if let Result::Output(out) = intputer.run() {
            out
        } else { panic!("wrong result") };
        assert_eq!(output, 1337);

        if let Result::Done = intputer.run() {} else { panic!("wrong result") };
    }

    #[test]
    fn test_simpler_run() {
        assert_eq!(Intputer::run_with_input_single_output("3,0,4,0,99", 1337).unwrap(), 1337);
        assert_eq!(Intputer::run_no_input_single_output("104,10,99").unwrap(), 10);

        assert_eq!(Intputer::run_with_input("3,0,4,0,99", 1337), vec![1337]);
        assert_eq!(Intputer::run_no_input("104,10,104,11,99"), vec![10, 11]);
    }
}
