use crate::intputer::Intputer;
use crate::read_lines::read_lines;

pub fn nine() {
    let mut input = read_lines("seven");
    let program = input.pop().unwrap();
    let intputer = Intputer::new(program.as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_data() {
        let program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let output = Intputer::run_no_input(program);

        println!("{:?}", output);

//        assert_eq!(Intputer::run_no_input_single_output(program).unwrap(), 1337);
    }

    #[test]
    fn test_relative_base() {
        assert_eq!(Intputer::run_no_input_single_output("204,0,99").unwrap(), 204);
        assert_eq!(Intputer::run_no_input_single_output("204,1,99").unwrap(), 1);
        assert_eq!(Intputer::run_no_input_single_output("9,1,22201,0,1,-1,204,-1,99").unwrap(), 22202);
    }
}
