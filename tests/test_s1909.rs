#[cfg(test)]
mod test_s1909 {
    use leetcode::s1909_remove_one_element_to_make_the_array_strictly_increasing::can_be_increasing;

    #[test]
    fn test_case_1() {
        assert_eq!(can_be_increasing(vec![1, 2, 10, 5, 7]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_be_increasing(vec![2, 3, 1, 1]), false);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(can_be_increasing(vec![1, 1, 1]), false);
    }
}
