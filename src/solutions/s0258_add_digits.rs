pub fn add_digits(num: i32) -> i32 {
    let mut sum = num;

    while sum > 9 {
        sum = sum
            .to_string()
            .chars()
            .map(|digit| digit.to_digit(10 as u32).unwrap())
            .sum::<u32>() as i32
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(add_digits(38), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(add_digits(0), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(add_digits(10), 1);
    }
}
