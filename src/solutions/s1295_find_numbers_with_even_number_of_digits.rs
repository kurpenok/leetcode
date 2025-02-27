pub fn find_numbers(nums: Vec<i32>) -> i32 {
    nums.iter()
        .filter(|num| num.to_string().len() % 2 == 0)
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_numbers(vec![555, 901, 482, 1771]), 1);
    }
}
