#[cfg(test)]
mod test_s1791 {
    use leetcode::s1791_find_center_of_star_graph::find_center;

    #[test]
    fn test_case_1() {
        assert_eq!(find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
            1
        );
    }
}
