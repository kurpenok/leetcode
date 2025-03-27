pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = *candies.iter().max().unwrap_or(&0);
    candies
        .iter()
        .map(|&c| c + extra_candies >= max_candies)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            [true, true, true, false, true]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            [true, false, false, false, false]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(kids_with_candies(vec![12, 1, 12], 10), [true, false, true]);
    }
}
