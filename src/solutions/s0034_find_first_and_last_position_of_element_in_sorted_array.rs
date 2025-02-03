pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if let Ok(_) = nums.binary_search(&target) {
        vec![
            nums.partition_point(|&i| i < target) as i32,
            nums.partition_point(|&i| i <= target) as i32 - 1,
        ]
    } else {
        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), [3, 4]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 6), [-1, -1]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(search_range(vec![], 0), [-1, -1]);
    }
}
