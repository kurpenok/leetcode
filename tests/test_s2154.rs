#[cfg(test)]
mod test_s2154 {
    use leetcode::s2154_keep_multiplying_found_values_by_two::find_final_value;

    #[test]
    fn test_case_1() {
        assert_eq!(find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_final_value(vec![2, 7, 9], 4), 4);
    }
}
