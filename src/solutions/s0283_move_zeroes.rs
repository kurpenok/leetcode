pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut left_zero = nums.len();
    let mut right_zero = nums.len();

    while i < nums.len() {
        if left_zero == nums.len() && right_zero == nums.len() && nums[i] == 0 {
            left_zero = i;
            right_zero = i;
        } else if left_zero == nums.len() && right_zero == nums.len() && nums[i] != 0 {
            i += 1;
            continue;
        } else if nums[i] == 0 {
            right_zero += 1;
        } else if nums[i] != 0 {
            nums[left_zero] = nums[i];
            left_zero += 1;
            right_zero += 1;
            nums[i] = 0;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums: Vec<i32> = vec![0];
        move_zeroes(&mut nums);
        assert_eq!(vec![0], nums);
    }

    #[test]
    fn test_case_2() {
        let mut nums: Vec<i32> = vec![1];
        move_zeroes(&mut nums);
        assert_eq!(vec![1], nums);
    }

    #[test]
    fn test_case_3() {
        let mut nums: Vec<i32> = vec![1, 0];
        move_zeroes(&mut nums);
        assert_eq!(vec![1, 0], nums);
    }

    #[test]
    fn test_case_4() {
        let mut nums: Vec<i32> = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);
    }
}
