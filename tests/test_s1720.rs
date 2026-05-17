#[cfg(test)]
mod test_s1720 {
    use leetcode::s1720_decode_xored_array::decode;

    #[test]
    fn test_case_1() {
        assert_eq!(decode(vec![1, 2, 3], 1), [1, 0, 2, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(decode(vec![6, 2, 7, 3], 4), [4, 2, 0, 7, 4]);
    }
}
