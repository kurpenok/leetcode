pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut index_of_max: usize = 0;
    let mut max: i32 = nums[index_of_max];
    let mut second_max: i32 = nums[1];

    for i in 0..nums.len() {
        if max < nums[i] {
            second_max = max;
            max = nums[i];
            index_of_max = i;
            continue;
        }

        if nums[i] != max && second_max < nums[i] {
            second_max = nums[i];
        }
    }

    if max / 2 >= second_max {
        return index_of_max as i32;
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(dominant_index(vec![3, 6, 1, 0]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(dominant_index(vec![1, 2, 3, 4]), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(dominant_index(vec![1, 0]), 0);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(dominant_index(vec![0, 0, 3, 2]), -1);
    }
}
