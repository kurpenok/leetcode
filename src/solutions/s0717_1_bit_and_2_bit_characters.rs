pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
    let mut index: usize = 0;

    while index < bits.len() {
        if index == bits.len() - 1 {
            return true;
        }

        if bits[index] == 0 {
            index += 1;
        } else {
            index += 2;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_one_bit_character(vec![1, 0, 0]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(is_one_bit_character(vec![1, 1, 1, 0]), false);
    }
}
