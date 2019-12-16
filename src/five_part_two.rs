use crate::intputer::Intputer;
use crate::read_lines::read_lines;

pub fn five() {
    let mut input = read_lines("five");
    let mut intputer = Intputer::with_input(input.pop().unwrap().as_str(), vec![5]);
    let result = intputer.legacy_run();
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::intputer::*;
    use crate::intputer::Result::Output;

    #[test]
    fn test_below_eight() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut intputer = Intputer::new(program);
        intputer.input(7);
        let output = match intputer.run() {
            Output(out) => out,
            _ => panic!("wrong status"),
        };

        assert_eq!(output, 999);
    }

    #[test]
    fn test_equal_eight() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut intputer = Intputer::new(program);
        intputer.input(8);
        let output = match intputer.run() {
            Output(out) => out,
            _ => panic!("wrong status"),
        };

        assert_eq!(output, 1000);
    }

    #[test]
    fn test_greater_than_eight() {
        let program = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut intputer = Intputer::new(program);
        intputer.input(9);
        let output = match intputer.run() {
            Output(out) => out,
            _ => panic!("wrong status"),
        };

        assert_eq!(output, 1001);
    }
}
