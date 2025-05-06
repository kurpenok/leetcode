pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    if nums.len() == 1 {
        return nums[0];
    }

    let mut prev_prev = nums[0];
    let mut prev = nums[0].max(nums[1]);

    for i in 2..nums.len() {
        let current = prev_prev + nums[i];
        prev_prev = prev;
        prev = current.max(prev);
    }

    prev
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    }
}
