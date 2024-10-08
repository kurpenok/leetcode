pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                left += 1;
                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                }
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            [[-1, -1, 2], [-1, 0, 1]]
        );
    }

    #[test]
    fn test_case_2() {
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(three_sum(vec![0, 1, 1]), expected);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(three_sum(vec![0, 0, 0]), [[0, 0, 0]]);
    }
}
