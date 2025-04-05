pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.into_iter()
        .fold([0, 0], |[a, b], x| [a ^ x & !b, a & x | b & !x])[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(single_number(vec![2, 2, 3, 2]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(single_number(vec![0, 1, 0, 1, 0, 1, 99]), 99);
    }
}
