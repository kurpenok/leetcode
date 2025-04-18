pub fn maximum_gap(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let mut nums = nums;
    nums.sort();

    let mut result: i32 = 0;
    for i in 1..nums.len() {
        result = std::cmp::max(result, (nums[i - 1] - nums[i]).abs());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(maximum_gap(vec![3, 6, 9, 1]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(maximum_gap(vec![10]), 0);
    }
}
