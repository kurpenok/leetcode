pub fn pivot_index(nums: Vec<i32>) -> i32 {
    for i in 0..nums.len() {
        if &nums[..i].iter().sum::<i32>() == &nums[i + 1..].iter().sum::<i32>() {
            return i as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    }
}
