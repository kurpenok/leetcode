#[cfg(test)]
mod test_s2144 {
    use leetcode::s2144_minimum_cost_of_buying_candies_with_discount::minimum_cost;

    #[test]
    fn test_case_1() {
        assert_eq!(minimum_cost(vec![1, 2, 3]), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(minimum_cost(vec![6, 5, 7, 9, 2, 2]), 23);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(minimum_cost(vec![5, 5]), 10);
    }
}
