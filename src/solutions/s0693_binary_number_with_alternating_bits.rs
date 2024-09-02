pub fn has_alternating_bits(n: i32) -> bool {
    let binary_digits = format!("{:b}", n).chars().collect::<Vec<char>>();

    for i in 1..binary_digits.len() {
        if binary_digits[i - 1] == binary_digits[i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(has_alternating_bits(5), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(has_alternating_bits(7), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(has_alternating_bits(11), false);
    }
}
