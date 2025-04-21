pub fn trailing_zeroes(n: i32) -> i32 {
    let mut n = n;

    let mut count: i32 = 0;
    while n >= 5 {
        count += n / 5;
        n /= 5;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(trailing_zeroes(3), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(trailing_zeroes(5), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(trailing_zeroes(0), 0);
    }
}
