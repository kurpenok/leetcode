pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    nums.chunks(2)
        .flat_map(|chunk| vec![chunk[1]; chunk[0] as usize])
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(decompress_rl_elist(vec![1, 2, 3, 4]), [2, 4, 4, 4]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(decompress_rl_elist(vec![1, 1, 2, 3]), [1, 3, 3]);
    }
}
