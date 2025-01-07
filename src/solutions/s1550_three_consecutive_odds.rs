pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
    if arr.len() > 2 {
        for i in 2..arr.len() {
            if arr[i - 2] % 2 == 1 && arr[i - 1] % 2 == 1 && arr[i] % 2 == 1 {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(three_consecutive_odds(vec![2, 6, 4, 1]), false);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            three_consecutive_odds(vec![1, 2, 34, 3, 4, 5, 7, 23, 12]),
            true,
        );
    }
}
