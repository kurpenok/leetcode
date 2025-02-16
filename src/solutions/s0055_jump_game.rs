pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut furthest_reachable = 0;

    for i in 0..nums.len() - 1 {
        furthest_reachable = std::cmp::max(furthest_reachable, i + nums[i] as usize);
        if furthest_reachable == i {
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
        assert_eq!(can_jump(vec![2, 3, 1, 1, 4]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
