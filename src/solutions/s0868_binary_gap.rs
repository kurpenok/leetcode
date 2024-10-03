pub fn binary_gap(n: i32) -> i32 {
    let mut result = usize::MIN;
    let mut prev: Option<usize> = None;

    for (i, digit) in format!("{:b}", n).char_indices() {
        if digit == '1' {
            if let Some(x) = prev {
                result = result.max(i - x);
            }
            prev = Some(i);
        }
    }

    result as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(binary_gap(22), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(binary_gap(8), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(binary_gap(5), 2);
    }
}
