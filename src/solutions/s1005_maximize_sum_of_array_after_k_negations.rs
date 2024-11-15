pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut negations_count = 0;
    let mut i = 0;

    while i < nums.len() && negations_count < k {
        if nums[i] >= 0 {
            break;
        }
        nums[i] = -nums[i];
        negations_count += 1;
        i += 1;
    }

    nums.sort();
    if negations_count < k {
        nums[0] = nums[0] * (-1i32).pow((k - negations_count) as u32);
    }

    nums.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(largest_sum_after_k_negations(vec![4, 2, 3], 1), 5);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(largest_sum_after_k_negations(vec![3, -1, 0, 2], 3), 6);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(largest_sum_after_k_negations(vec![2, -3, -1, 5, -4], 2), 13);
    }
}
