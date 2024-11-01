use std::collections::HashSet;

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut unique_nums: HashSet<i32> = HashSet::new();

    for num in &nums {
        if unique_nums.contains(num) {
            return *num;
        }
        unique_nums.insert(*num);
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(repeated_n_times(vec![1, 2, 3, 3]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(repeated_n_times(vec![2, 1, 2, 5, 3, 2]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(repeated_n_times(vec![5, 1, 5, 2, 5, 3, 5, 4]), 5);
    }
}
