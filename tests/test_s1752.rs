#[cfg(test)]
mod test_s1752 {
    use leetcode::s1752_check_if_array_is_sorted_and_rotated::check;

    #[test]
    fn test_case_1() {
        assert_eq!(check(vec![3, 4, 5, 1, 2]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(check(vec![2, 1, 3, 4]), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(check(vec![1, 2, 3]), true);
    }
}
