#[cfg(test)]
mod test_s1848 {
    use leetcode::s1848_minimum_distance_to_the_target_element::get_min_distance;

    #[test]
    fn test_case_1() {
        assert_eq!(get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(get_min_distance(vec![1], 1, 0), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
            0
        );
    }
}
