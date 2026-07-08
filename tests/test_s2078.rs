#[cfg(test)]
mod test_s2078 {
    use leetcode::s2078_two_furthest_houses_with_different_colors::max_distance;

    #[test]
    fn test_case_1() {
        assert_eq!(max_distance(vec![1, 1, 1, 6, 1, 1, 1]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_distance(vec![1, 8, 3, 8, 3]), 4);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_distance(vec![0, 1]), 1);
    }
}
