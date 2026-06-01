#[cfg(test)]
mod test_s1822 {
    use leetcode::s1822_sign_of_the_product_of_an_array::array_sign;

    #[test]
    fn test_case_1() {
        assert_eq!(array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(array_sign(vec![1, 5, 0, 2, -3]), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(array_sign(vec![-1, 1, -1, 1, -1]), -1);
    }
}
