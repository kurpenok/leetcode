pub fn count_odds(low: i32, high: i32) -> i32 {
    (high + 1) / 2 - low / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_odds(3, 7), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_odds(8, 10), 1);
    }
}
