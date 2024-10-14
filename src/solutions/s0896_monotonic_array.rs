fn is_increase_sorted(nums: &Vec<i32>) -> bool {
    for i in 1..nums.len() {
        if nums[i - 1] > nums[i] {
            return false;
        }
    }
    true
}

fn is_decrease_sorted(nums: &Vec<i32>) -> bool {
    for i in 1..nums.len() {
        if nums[i - 1] < nums[i] {
            return false;
        }
    }
    true
}

pub fn is_monotonic(nums: Vec<i32>) -> bool {
    is_increase_sorted(&nums) || is_decrease_sorted(&nums)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_monotonic(vec![1, 2, 2, 3]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_monotonic(vec![6, 5, 4, 4]), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(is_monotonic(vec![1, 3, 2]), false);
    }
}
