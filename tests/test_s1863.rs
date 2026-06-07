#[cfg(test)]
mod test_s1863 {
    use leetcode::s1863_sum_of_all_subset_xor_totals::subset_xor_sum;

    #[test]
    fn test_case_1() {
        assert_eq!(subset_xor_sum(vec![1, 3]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(subset_xor_sum(vec![5, 1, 6]), 28);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(subset_xor_sum(vec![3, 4, 5, 6, 7, 8]), 480);
    }
}
