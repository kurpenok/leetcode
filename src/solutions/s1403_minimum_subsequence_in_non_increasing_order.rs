pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(a));

    for i in 0..=nums.len() {
        if nums[..i].iter().sum::<i32>() > nums[i..].iter().sum::<i32>() {
            return nums[..i].to_vec();
        }
    }

    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(min_subsequence(vec![4, 3, 10, 9, 8]), [10, 9]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_subsequence(vec![4, 4, 7, 6, 7]), [7, 7, 6]);
    }
}
