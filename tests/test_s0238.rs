#[cfg(test)]
mod test_s0238 {
    use leetcode::s0238_product_of_array_except_self::product_except_self;

    #[test]
    fn test_case_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), [24, 12, 8, 6]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
    }
}
