#[cfg(test)]
mod test_s1725 {
    use leetcode::s1725_number_of_rectangles_that_can_form_the_largest_square::count_good_rectangles;

    #[test]
    fn test_case_1() {
        assert_eq!(
            count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
            3
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
            3
        );
    }
}
