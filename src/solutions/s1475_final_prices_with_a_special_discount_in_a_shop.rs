pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    prices
        .iter()
        .enumerate()
        .map(|(i, &price)| {
            price
                - prices
                    .iter()
                    .skip(i + 1)
                    .find(|&&x| x <= price)
                    .copied()
                    .unwrap_or(0)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(final_prices(vec![8, 4, 6, 2, 3]), [4, 2, 4, 2, 3]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(final_prices(vec![1, 2, 3, 4, 5]), [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(final_prices(vec![10, 1, 1, 6]), [9, 0, 1, 6]);
    }
}
