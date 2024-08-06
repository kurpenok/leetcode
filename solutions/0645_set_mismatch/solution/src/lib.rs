pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();

    let mut duplicate: i32 = 0;
    for i in 1..nums.len() {
        if nums[i - 1] == nums[i] {
            duplicate = nums[i];
            break;
        }
    }

    let sum_nums = nums.iter().sum::<i32>() - duplicate;
    let true_sum_nums = ((nums.len() * (nums.len() + 1)) / 2) as i32;

    vec![duplicate, true_sum_nums - sum_nums]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_error_nums(vec![1, 1]), vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_error_nums(vec![2, 2]), vec![2, 1]);
    }
}
