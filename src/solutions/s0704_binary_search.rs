pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = nums.len();

    while left < right {
        let middle: usize = left + (right - left) / 2;
        match nums[middle].cmp(&target) {
            std::cmp::Ordering::Equal => return middle as i32,
            std::cmp::Ordering::Less => left = middle + 1,
            std::cmp::Ordering::Greater => right = middle,
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1);
    }
}
