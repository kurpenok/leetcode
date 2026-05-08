#[cfg(test)]
mod test_s1652 {
    use leetcode::s1652_defuse_the_bomb::decrypt;

    #[test]
    fn test_case_1() {
        assert_eq!(decrypt(vec![5, 7, 1, 4], 3), vec![12, 10, 16, 13]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(decrypt(vec![1, 2, 3, 4], 0), vec![0, 0, 0, 0]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(decrypt(vec![2, 4, 9, 3], -2), vec![12, 5, 6, 13]);
    }
}
