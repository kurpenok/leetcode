pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let mut max_average: f64 = f64::MIN;

    for i in 0..nums.len() {
        if i + k as usize <= nums.len() {
            let subsequence = &nums[i..(i + k as usize)];
            let average = subsequence.iter().sum::<i32>() as f64 / subsequence.len() as f64;

            if max_average < average {
                max_average = average;
            }
        }
    }

    max_average
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_max_average(vec![1, 12, -5, -6, 50, 3], 4), 12.75);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_max_average(vec![5], 1), 5.0);
    }
}
