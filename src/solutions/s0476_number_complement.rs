pub fn find_complement(num: i32) -> i32 {
    let mut num = num;
    let bits_count = ((num as f64).log2() + 1.0) as usize;

    for i in 0..bits_count {
        num = num ^ (1 << i);
    }

    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_complement(5), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_complement(1), 0);
    }
}
