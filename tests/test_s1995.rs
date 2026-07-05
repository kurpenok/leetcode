#[cfg(test)]
mod test_s1995 {
    use leetcode::s1995_count_special_quadruplets::count_quadruplets;

    #[test]
    fn test_case_1() {
        assert_eq!(count_quadruplets(vec![1, 2, 3, 6]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
    }
}
