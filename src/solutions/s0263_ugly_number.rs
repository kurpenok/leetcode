pub fn is_ugly(n: i32) -> bool {
    if n <= 0 {
        return false;
    }

    let mut n = n;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
            continue;
        } else if n % 3 == 0 {
            n /= 3;
            continue;
        } else if n % 5 == 0 {
            n /= 5;
            continue;
        } else {
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
        assert_eq!(is_ugly(6), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_ugly(1), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_ugly(14), false);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(is_ugly(-2147483648), false);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(is_ugly(0), false);
    }
}
