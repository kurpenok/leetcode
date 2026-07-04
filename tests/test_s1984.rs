#[cfg(test)]
mod test_s1984 {
    use leetcode::s1984_minimum_difference_between_highest_and_lowest_of_k_scores::minimum_difference;

    #[test]
    fn test_case_1() {
        assert_eq!(minimum_difference(vec![90], 1), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
