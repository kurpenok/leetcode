#[cfg(test)]
mod test_s1779 {
    use leetcode::s1779_find_nearest_point_that_has_the_same_x_or_y_coordinate::nearest_valid_point;

    #[test]
    fn test_case_1() {
        assert_eq!(
            nearest_valid_point(
                3,
                4,
                vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]
            ),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
    }
}
