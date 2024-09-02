pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut max_sum: i32 = 0;
    for i in 0..nums.len() {
        if i % 2 == 0 {
            max_sum += nums[i];
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(array_pair_sum(vec![1, 4, 3, 2]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(array_pair_sum(vec![6, 2, 6, 5, 1, 2]), 9);
    }
}
