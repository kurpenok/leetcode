pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    nums.iter()
        .take(n as usize)
        .zip(&nums[n as usize..])
        .flat_map(|(x, y)| vec![*x, *y])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(shuffle(vec![2, 5, 1, 3, 4, 7], 3), [2, 3, 5, 4, 1, 7]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            [1, 4, 2, 3, 3, 2, 4, 1]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(shuffle(vec![1, 1, 2, 2], 2), [1, 2, 1, 2]);
    }
}
