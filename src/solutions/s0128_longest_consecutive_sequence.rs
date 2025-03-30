use std::collections::HashSet;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let sequence: HashSet<i32> = nums.into_iter().collect();
    let mut max_length: i32 = 0;

    for n in &sequence {
        let mut n_copy = n.clone();

        if !sequence.contains(&(&n_copy - 1)) {
            let mut current_length = 1;

            while sequence.contains(&(&n_copy + 1)) {
                current_length += 1;
                n_copy += 1;
            }

            max_length = std::cmp::max(current_length, max_length);
        }
    }

    max_length
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(longest_consecutive(vec![1, 0, 1, 2]), 3);
    }
}
