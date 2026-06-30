#[cfg(test)]
mod test_s1920 {
    use leetcode::s1920_build_array_from_permutation::build_array;

    #[test]
    fn test_case_1() {
        assert_eq!(build_array(vec![0, 2, 1, 5, 3, 4]), [0, 1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(build_array(vec![5, 0, 1, 2, 3, 4]), [4, 5, 0, 1, 2, 3]);
    }
}
