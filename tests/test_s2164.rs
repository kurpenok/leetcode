#[cfg(test)]
mod test_s2164 {
    use leetcode::s2164_sort_even_and_odd_indices_independently::sort_even_odd;

    #[test]
    fn test_case_1() {
        assert_eq!(sort_even_odd(vec![4, 1, 2, 3]), [2, 3, 4, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sort_even_odd(vec![2, 1]), [2, 1]);
    }
}
