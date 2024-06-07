pub fn is_power_of_four(n: i32) -> bool {
    if n < 0 {
        return false;
    }

    let log = (n as f64).log2();
    log % 2.0 == 0.0 && log.trunc() == log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_power_of_four(16), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_power_of_four(5), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_power_of_four(1), true);
    }
}
