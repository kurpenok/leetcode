#[cfg(test)]
mod test_s0216 {
    use leetcode::s0216_combination_sum_iii::combination_sum3;

    #[test]
    fn test_case_1() {
        assert_eq!(combination_sum3(3, 7), [[1, 2, 4]]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(combination_sum3(3, 9), [[1, 2, 6], [1, 3, 5], [2, 3, 4]]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(combination_sum3(4, 1), Vec::<Vec<i32>>::new());
    }
}
