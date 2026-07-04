#[cfg(test)]
mod test_s1979 {
    use leetcode::s1979_find_greatest_common_divisor_of_array::find_gcd;

    #[test]
    fn test_case_1() {
        assert_eq!(find_gcd(vec![2, 5, 6, 9, 10]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_gcd(vec![7, 5, 6, 8, 3]), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_gcd(vec![3, 3]), 3);
    }
}
