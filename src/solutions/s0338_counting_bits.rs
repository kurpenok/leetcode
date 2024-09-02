pub fn count_bits(n: u32) -> Vec<i32> {
    let mut ans: Vec<i32> = vec![0; n as usize + 1];

    for i in 0..=n {
        ans[i as usize] = i.count_ones() as i32;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
