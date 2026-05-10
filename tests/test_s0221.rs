#[cfg(test)]
mod test_s0221 {
    use leetcode::s0221_maximal_square::maximal_square;

    #[test]
    fn test_case_1() {
        assert_eq!(
            maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximal_square(vec![vec!['0', '1'], vec!['1', '0']]), 1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(maximal_square(vec![vec!['0']]), 0);
    }
}
