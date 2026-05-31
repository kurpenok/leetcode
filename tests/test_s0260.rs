#[cfg(test)]
mod test_s0260 {
    use leetcode::s0260_single_number_iii::single_number;

    #[test]
    fn test_case_1() {
        assert_eq!(single_number(vec![1, 2, 1, 3, 2, 5]), [3, 5]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(single_number(vec![-1, 0]), [-1, 0]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(single_number(vec![0, 1]), [1, 0]);
    }
}
