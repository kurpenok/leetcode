pub fn jump(nums: Vec<i32>) -> i32 {
    let mut jumps = 0;
    let mut max_distance = 0;
    let mut current_position = 0;

    for i in 0..nums.len() {
        if i == nums.len() - 1 {
            break;
        }

        max_distance = std::cmp::max(max_distance, nums[i] + i as i32);

        if current_position == i as i32 {
            jumps += 1;
            current_position = max_distance;
        }
    }

    jumps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
