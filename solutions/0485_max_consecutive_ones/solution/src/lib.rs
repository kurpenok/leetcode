pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut max_ones_seq_len: i32 = 0;
    let mut ones_seq_len: i32 = 0;

    for num in nums {
        if num == 1 {
            ones_seq_len += 1;
            if max_ones_seq_len < ones_seq_len {
                max_ones_seq_len = ones_seq_len;
            }
        } else {
            ones_seq_len = 0;
        }
    }

    max_ones_seq_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
