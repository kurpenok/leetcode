pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut best_diff = i32::max_value();

    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        let mut left = i + 1;
        let mut right = nums.len() - 1;

        let prev_diff = nums[i] - target;

        while left < right {
            let diff = prev_diff + nums[left] + nums[right];
            if diff.abs() < best_diff.abs() {
                best_diff = diff;
            }

            match diff.signum() {
                1 => right -= 1,
                -1 => left += 1,
                0 => return target,
                _ => unreachable!(),
            }
        }
    }

    target + best_diff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(three_sum_closest(vec![0, 0, 0], 1), 0);
    }
}
