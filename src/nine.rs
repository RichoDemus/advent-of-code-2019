use crate::intputer::Intputer;
use crate::read_lines::read_lines;

pub fn nine() -> i64 {
    let mut input = read_lines("nine");
    let program = input.pop().unwrap();
    Intputer::run_with_input_single_output(program.as_str(), 1).unwrap()
}

pub fn nine_part2() -> i64 {
    let mut input = read_lines("nine");
    let program = input.pop().unwrap();
    Intputer::run_with_input_single_output(program.as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn sample_data() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let output = Intputer::run_no_input(program);
        assert_eq!(output.iter().join(","), program);

        assert_eq!(Intputer::run_no_input_single_output("1102,34915192,34915192,7,4,7,99,0").unwrap(), 1219070632396864);
        assert_eq!(Intputer::run_no_input_single_output("104,1125899906842624,99").unwrap(), 1125899906842624);

    }

    #[test]
    fn test_relative_base() {
        assert_eq!(Intputer::run_no_input_single_output("204,0,99").unwrap(), 204);
        assert_eq!(Intputer::run_no_input_single_output("204,1,99").unwrap(), 1);
        assert_eq!(Intputer::run_no_input_single_output("9,1,22201,0,1,-1,204,-1,99").unwrap(), 22202);
    }

    #[test]
    fn solve_nine() {
        assert_eq!(nine(), 3013554615);
    }

    #[test]
    fn solve_nine_part2() {
        assert_eq!(nine_part2(), 50158);
    }
}
