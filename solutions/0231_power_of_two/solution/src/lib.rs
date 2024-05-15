pub fn is_power_of_two(n: i32) -> bool {
    if n == 0 {
        return false;
    }

    let log = (n as f64).log2();
    log.trunc() == log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_power_of_two(1), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_power_of_two(16), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_power_of_two(3), false);
    }
}
