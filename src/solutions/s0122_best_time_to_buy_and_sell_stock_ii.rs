pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit: i32 = 0;
    let mut buy: i32 = prices[0];

    for i in 1..prices.len() {
        if buy < prices[i] {
            profit += prices[i] - buy;
        }

        buy = prices[i];
    }

    profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
