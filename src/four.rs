use std::intrinsics::transmute;

pub fn four() {
    println!("four, there are {} different passwords", num_different_passwords(372304, 847060));
}

fn num_different_passwords(start: u32, end: u32) -> usize {
    let mut valid_passwords = 0;
    for i in start..=end {
        if is_valid(i) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn is_valid(password: u32) -> bool {
    let digits = number_to_digits(password);

    // should be 6 digits
    if digits.len() != 6 {
        return false;
    }

    // should have at least set of identical adjacent digits
//    let pairs = adjacent_pairs(digits);
//    let mut pair_found = false;
//    for (left, right) in pairs.clone() {
//        if left == right {
//            pair_found = true;
//            break
//        }
//    }
//    if pair_found == false {
//        return false
//    }
    let mut pair_found = false;
    let mut identicals_found_so_far = 1;
    let mut last_digit = digits.get(0);
    for i in 1..=digits.len() {
        let current_digit = digits.get(i);
        if last_digit == current_digit {
            identicals_found_so_far += 1;
        } else {
            // we have a new digit, the last ones might have been a pair
            if identicals_found_so_far == 2 {
                pair_found = true;
                break;
            }
            last_digit = current_digit;
            identicals_found_so_far = 1;
        }
    }
    if pair_found == false {
        return false
    }

    //digits should never decrease
    for (left, right) in adjacent_pairs(digits) {
        if left > right {
            return false
        }
    }

    true
}

fn number_to_digits(number: u32) -> Vec<u32> {
    let digits: Vec<_> = number.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    digits
}

fn adjacent_pairs<T>(vector: Vec<T>) -> Vec<(T, T)> where T: std::fmt::Debug, T: Clone {
    let mut pairs: Vec<(T, T)> = vec![];

    let mut iterator = vector.into_iter().peekable();

    loop {
        match (iterator.next(), iterator.peek()) {
            (Some(left), Some(right)) => {
                let right: T = right.clone();
                pairs.push((left, right))
            }
            _ => {
                break;
            }
        }
    }
    pairs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        num_different_passwords(10, 20);
    }

    #[test]
    fn test_is_valid() {
        // six digits
        assert_eq!(is_valid(11234), false);
        assert_eq!(is_valid(112345), true);
        assert_eq!(is_valid(1123456), false);

        // pair
        assert_eq!(is_valid(123456), false, "fail if no adjacent");
        assert_eq!(is_valid(123455), true, "ok if one adjacent");
        assert_eq!(is_valid(123355), true, "ok if two adjacent");

        // never decrease
        assert_eq!(is_valid(112233), true, "should increase");
        assert_eq!(is_valid(112230), false, "0 is smaller than 3, should fail");

        // the adjacent pair should only be a pair
        assert_eq!(is_valid(112233), true);
        assert_eq!(is_valid(123444), false, "only a three of a kind should not be valid");
        assert_eq!(is_valid(111122), true, "111122 has a pair");
    }

    #[test]
    fn test_digits() {
        assert_eq!(number_to_digits(1234), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_pairs() {
        assert_eq!(adjacent_pairs(vec![1, 2]), vec![(1, 2)]);
        assert_eq!(adjacent_pairs(vec![1, 2, 3]), vec![(1, 2), (2, 3)]);
        assert_eq!(adjacent_pairs(vec![1, 2, 3, 4]), vec![(1, 2), (2, 3), (3, 4)]);
    }
}
