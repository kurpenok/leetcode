pub fn find_peak_element(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 || nums[0] > nums[1] {
        return 0;
    }

    if nums[nums.len() - 1] > nums[nums.len() - 2] {
        return nums.len() as i32 - 1;
    }

    for i in 1..nums.len() - 1 {
        if nums[i - 1] < nums[i] && nums[i] > nums[i + 1] {
            return i as i32;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_peak_element(vec![1, 2, 3, 1]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]), 1);
    }
}
