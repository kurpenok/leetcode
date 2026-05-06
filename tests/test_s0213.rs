#[cfg(test)]
mod test_s0213 {
    use leetcode::s0213_house_robber_ii::rob;

    #[test]
    fn test_case_1() {
        assert_eq!(rob(vec![2, 3, 2]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(rob(vec![1, 2, 3]), 3);
    }
}
