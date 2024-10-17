pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
    let mut min = nums[0];
    let mut max = nums[0];

    for num in nums {
        min = std::cmp::min(num, min);
        max = std::cmp::max(num, max);
    }

    std::cmp::max(max - min - 2 * k, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(smallest_range_i(vec![1], 0), 0);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(smallest_range_i(vec![0, 10], 2), 6);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(smallest_range_i(vec![1, 3, 6], 3), 0);
    }
}
