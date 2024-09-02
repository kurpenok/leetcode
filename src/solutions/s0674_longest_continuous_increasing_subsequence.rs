pub fn fing_length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut max_length_of_lcis: i32 = 1;
    let mut length_of_lcis: i32 = 1;

    for i in 1..nums.len() {
        println!("{} {}", nums[i - 1], nums[i]);
        if nums[i - 1] < nums[i] {
            length_of_lcis += 1;
        } else {
            max_length_of_lcis = std::cmp::max(max_length_of_lcis, length_of_lcis);
            length_of_lcis = 1;
        }
    }

    std::cmp::max(max_length_of_lcis, length_of_lcis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(fing_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(fing_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
    }
}
