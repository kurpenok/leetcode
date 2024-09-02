pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32 + 1;

    (((n - 1) * n) / 2) - nums.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(missing_number(vec![0, 1]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8);
    }
}
