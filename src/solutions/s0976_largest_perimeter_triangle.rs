pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort_unstable();

    for i in (2..nums.len()).rev() {
        if nums[i - 2] + nums[i - 1] > nums[i] {
            return nums[i - 2] + nums[i - 1] + nums[i];
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(largest_perimeter(vec![2, 1, 2]), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(largest_perimeter(vec![1, 2, 1, 10]), 0);
    }
}
