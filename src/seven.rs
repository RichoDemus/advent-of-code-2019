use std::cmp;

use itertools::Itertools;

use crate::intputer::Intputer;
use crate::read_lines::read_lines;
use crate::intputer::Result::Output;

pub fn seven() {
    let mut input = read_lines("seven");
    let program = input.pop().unwrap();
    let program = program.as_str();
    let result = highest_thruster_signal(program);
    println!("the highest thruster sinal is {}", result);
}

pub fn seven_part2() {
//    let result = run_feedback_loop_until_all_halt();
}

fn run_feedback_loop_until_all_halt(program: &str, sequence: Vec<i32>) -> i32 {
    println!("just print these to please linter: {:?}{:?}", program, sequence);
//    let intputers = vec![
//
//    ];
    -1
}

fn highest_thruster_signal(program: &str) -> i32 {
    let phases = vec![0, 1, 2, 3, 4];
    let permutations = permutations(phases);

    let mut record = 0;
    for phase_sequence in permutations {
        record = cmp::max(record, calculate_thruster_signal(program, phase_sequence));
    };
    record
}

fn calculate_thruster_signal(program: &str, sequence: Vec<i32>) -> i32 {
    fn get_output(program: &str, phase: i32, input: i32) -> i32 {
        let mut intputer = Intputer::new(program);
        intputer.input(phase);
        intputer.input(input);
        match intputer.run() {
            Output(out) => out,
            _ => panic!("wrong status"),
        }
    }

    let mut prev_output = 0;
    for phase in sequence {
        prev_output = get_output(program, phase, prev_output);
    };
    prev_output
}

fn permutations<T>(vec: Vec<T>) -> Vec<Vec<T>> where T: Clone {
    let len = vec.len();
    let permutations = vec.into_iter().permutations(len);
    permutations.collect()
}

#[cfg(test)]
mod tests {
    use crate::seven::{calculate_thruster_signal, highest_thruster_signal, permutations, run_feedback_loop_until_all_halt};

    #[test]
    fn test() {
        assert_eq!(highest_thruster_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);
        assert_eq!(highest_thruster_signal("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);
        // wtf is code 1007?
        assert_eq!(highest_thruster_signal("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
    }

    #[test]
    fn test_specific() {
        assert_eq!(calculate_thruster_signal("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_permutations() {
        assert_eq!(permutations(vec![1]), vec![vec![1]]);
        assert_eq!(permutations(vec![1, 2]), vec![
            vec![1, 2],
            vec![2, 1]
        ]);
        assert_eq!(permutations(vec![1, 2, 3]), vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ]);
    }

    #[test]
    fn test_feedback_loop() {
        assert_eq!(run_feedback_loop_until_all_halt("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0", vec![4, 3, 2, 1, 0]), 43210);
        assert_eq!(run_feedback_loop_until_all_halt("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0", vec![0, 1, 2, 3, 4]), 54321);
        assert_eq!(run_feedback_loop_until_all_halt("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0", vec![1, 0, 4, 3, 2]), 65210);
    }
}
