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
    let pairs = adjacent_pairs(digits);
    let mut pair_found = false;
    for (left, right) in pairs.clone() {
        if left == right {
            pair_found = true;
            break
        }
    }
    if pair_found == false {
        return false
    }


    //digits should never decrease
    for (left, right) in pairs {
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
        assert_eq!(is_valid(11111), false);
        assert_eq!(is_valid(111111), true);
        assert_eq!(is_valid(1111111), false);

        // pair
        assert_eq!(is_valid(123456), false, "fail if no adjacent");
        assert_eq!(is_valid(123455), true, "ok if one adjacent");
        assert_eq!(is_valid(123355), true, "ok if two adjacent");
        assert_eq!(is_valid(123555), true, "ok if three of a kind");

        // never decrease
        assert_eq!(is_valid(112233), true, "should increase");
        assert_eq!(is_valid(112230), false, "0 is smaller than 3, should fail");
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
