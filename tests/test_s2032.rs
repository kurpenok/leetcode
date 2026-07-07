#[cfg(test)]
mod test_s2032 {
    use leetcode::s2032_two_out_of_three::two_out_of_three;

    #[test]
    fn test_case_1() {
        assert_eq!(
            two_out_of_three(vec![1, 1, 3, 2], vec![2, 3], vec![3]),
            [2, 3]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            two_out_of_three(vec![3, 1], vec![2, 3], vec![1, 2]),
            [1, 2, 3]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(two_out_of_three(vec![1, 2, 2], vec![4, 3, 3], vec![5]), []);
    }
}
