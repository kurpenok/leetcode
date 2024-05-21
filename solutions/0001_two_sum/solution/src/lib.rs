use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map: HashMap<i32, usize> = HashMap::new();

    for i in 0..nums.len() {
        nums_map.insert(nums[i], i);
    }

    for i in 0..nums.len() {
        if nums_map.contains_key(&(target - nums[i])) && i != nums_map[&(target - nums[i])] {
            return vec![i as i32, nums_map[&(target - nums[i])] as i32];
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;

        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;

        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        let nums: Vec<i32> = vec![3, 3];
        let target: i32 = 6;

        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
