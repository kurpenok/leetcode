pub fn find_min(nums: Vec<i32>) -> i32 {
    *nums.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
    }
}
