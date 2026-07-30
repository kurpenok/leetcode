#[cfg(test)]
mod test_s2190 {
    use leetcode::s2190_most_frequent_number_following_key_in_an_array::most_frequent;

    #[test]
    fn test_case_1() {
        assert_eq!(most_frequent(vec![1, 100, 200, 1, 100], 1), 100);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(most_frequent(vec![2, 2, 2, 2, 3], 2), 2);
    }
}
