pub fn is_power_of_three(n: i32) -> bool {
    n > 0 && 1162261467 % n == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_power_of_three(27), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_power_of_three(0), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_power_of_three(-1), false);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(is_power_of_three(243), true);
    }
}
