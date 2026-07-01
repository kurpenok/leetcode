#[cfg(test)]
mod test_s1929 {
    use leetcode::s1929_concatenation_of_array::get_concatenation;

    #[test]
    fn test_case_1() {
        assert_eq!(get_concatenation(vec![1, 2, 1]), [1, 2, 1, 1, 2, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            get_concatenation(vec![1, 3, 2, 1]),
            [1, 3, 2, 1, 1, 3, 2, 1]
        );
    }
}
