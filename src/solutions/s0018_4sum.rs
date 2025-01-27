pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }

    let mut nums = nums;
    nums.sort();

    let mut quadruplets: Vec<Vec<i32>> = Vec::new();

    for i in 0..nums.len() - 1 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        for j in i + 1..nums.len() - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                continue;
            }

            let mut left = j + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[i] as i64 + nums[j] as i64 + nums[left] as i64 + nums[right] as i64;

                if sum == target as i64 {
                    quadruplets.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if sum < target as i64 {
                    left += 1
                } else if sum > target as i64 {
                    right -= 1;
                }
            }
        }
    }

    quadruplets
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            [[-2, -1, 1, 2], [-2, 0, 0, 2], [-1, 0, 0, 1]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(four_sum(vec![2, 2, 2, 2, 2], 8), [[2, 2, 2, 2]]);
    }
}
