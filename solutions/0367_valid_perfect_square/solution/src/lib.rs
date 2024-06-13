use std::cmp::Ordering;

pub fn is_perfect_square(num: i32) -> bool {
    if num == 1 {
        return true;
    }

    let mut left = 1;
    let mut right = num;
    while left <= right {
        let middle = left + (right - left) / 2;
        match middle.checked_mul(middle) {
            Some(square) => match square.cmp(&num) {
                Ordering::Equal => return true,
                Ordering::Less => left = middle + 1,
                Ordering::Greater => right = middle - 1,
            },
            None => right = middle - 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_perfect_square(16), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_perfect_square(14), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_perfect_square(9), true);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(is_perfect_square(808201), true);
    }
}
