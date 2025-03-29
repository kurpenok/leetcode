pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut left_one: i32 = -1;
    let mut right_one: i32 = -1;

    for i in 0..nums.len() {
        if nums[i] == 1 && left_one == -1 {
            left_one = i as i32;
            continue;
        } else if nums[i] == 1 && right_one == -1 {
            right_one = i as i32;
        } else if nums[i] == 1 {
            left_one = right_one;
            right_one = i as i32;
        }

        if nums[i] == 1 && (left_one - right_one).abs() - 1 < k {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(k_length_apart(vec![1, 0, 0, 0, 1, 0, 0, 1], 2), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(k_length_apart(vec![1, 0, 0, 1, 0, 1], 2), false);
    }
}
