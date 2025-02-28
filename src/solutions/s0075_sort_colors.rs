pub fn sort_colors(nums: &mut Vec<i32>) {
    for i in 0..nums.len() {
        let mut swapped: bool = false;
        for j in 0..(nums.len() - i - 1) {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut nums: Vec<i32> = vec![2, 0, 2, 1, 1, 0];

        sort_colors(&mut nums);

        assert_eq!(nums, [0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_case_2() {
        let mut nums: Vec<i32> = vec![2, 0, 1];

        sort_colors(&mut nums);

        assert_eq!(nums, [0, 1, 2]);
    }
}
