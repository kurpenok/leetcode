#[cfg(test)]
mod test_s1742 {
    use leetcode::s1742_maximum_number_of_balls_in_a_box::count_balls;

    #[test]
    fn test_case_1() {
        assert_eq!(count_balls(1, 10), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_balls(5, 15), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(count_balls(19, 28), 2);
    }
}
