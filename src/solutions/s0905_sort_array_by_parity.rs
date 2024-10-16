pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut evens = Vec::new();
    let mut odds = Vec::new();

    for num in nums {
        if num % 2 == 0 {
            evens.push(num);
        } else {
            odds.push(num);
        }
    }
    evens.extend(odds);

    evens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(sort_array_by_parity(vec![3, 1, 2, 4]), [2, 4, 3, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sort_array_by_parity(vec![0]), [0]);
    }
}
