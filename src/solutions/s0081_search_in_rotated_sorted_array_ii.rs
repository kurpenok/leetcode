pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut left = 0;
    let mut right = nums.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if nums[mid as usize] == target {
            return true;
        }

        if nums[left as usize] == nums[mid as usize] {
            left += 1;
            continue;
        }

        if nums[left as usize] <= nums[mid as usize] {
            if nums[left as usize] <= target && target <= nums[mid as usize] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            if nums[mid as usize] <= target && target <= nums[right as usize] {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
    }

    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
    }
}
