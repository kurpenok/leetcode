pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let mut squares: Vec<i32> = nums.iter().map(|&n| n * n).collect();
    squares.sort_unstable();
    squares
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(sorted_squares(vec![-4, -1, 0, 3, 10]), [0, 1, 9, 16, 100]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sorted_squares(vec![-7, -3, 2, 3, 11]), [4, 9, 9, 49, 121]);
    }
}
