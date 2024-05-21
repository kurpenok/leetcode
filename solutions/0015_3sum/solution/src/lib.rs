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
        let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
        let expected: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];

        assert_eq!(three_sum(nums), expected);
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = vec![0, 1, 1];
        let expected: Vec<Vec<i32>> = vec![];

        assert_eq!(three_sum(nums), expected);
    }

    #[test]
    fn test_case_3() {
        let nums: Vec<i32> = vec![0, 0, 0];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]];

        assert_eq!(three_sum(nums), expected);
    }

    #[test]
    fn test_case_4() {
        let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
        let expected: Vec<Vec<i32>> = vec![
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-3, -1, 4],
            vec![-3, 0, 3],
            vec![-3, 1, 2],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, -1, 2],
            vec![-1, 0, 1],
        ];

        assert_eq!(three_sum(nums), expected);
    }
}
