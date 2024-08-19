pub fn count_binary_substrings(s: String) -> i32 {
    let symbols: Vec<char> = s.chars().collect();
    let mut counter: i32 = 0;

    // Count zeros
    let mut count_zeros: i32 = 0;
    let mut count_ones: i32 = 0;
    for s in &symbols {
        if *s == '0' && count_ones != 0 {
            count_zeros = 1;
            count_ones = 0;
        } else if *s == '0' {
            count_zeros += 1;
        } else if *s == '1' && count_zeros > 0 && count_ones < count_zeros {
            counter += 1;
            count_ones += 1;
        }
    }

    // Count ones
    let mut count_zeros = 0;
    let mut count_ones = 0;
    for s in &symbols {
        if *s == '1' && count_zeros != 0 {
            count_zeros = 0;
            count_ones = 1;
        } else if *s == '1' {
            count_ones += 1;
        } else if *s == '0' && count_ones > 0 && count_zeros < count_ones {
            counter += 1;
            count_zeros += 1;
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_binary_substrings("00110011".to_string()), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_binary_substrings("10101".to_string()), 4);
    }
}
