#[cfg(test)]
mod test_s1637 {
    use leetcode::s1637_widest_vertical_area_between_two_points_containing_no_points::max_width_of_vertical_area;

    #[test]
    fn test_case_1() {
        assert_eq!(
            max_width_of_vertical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]]),
            1
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            max_width_of_vertical_area(vec![
                vec![3, 1],
                vec![9, 0],
                vec![1, 0],
                vec![1, 4],
                vec![5, 3],
                vec![8, 8]
            ]),
            3
        );
    }
}
