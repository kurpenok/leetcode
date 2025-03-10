pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;

    arr.sort_by(|a, b| {
        let ones_a = a.count_ones();
        let ones_b = b.count_ones();

        match ones_a.cmp(&ones_b) {
            std::cmp::Ordering::Equal => a.cmp(b),
            ordering => ordering,
        }
    });

    arr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
            [0, 1, 2, 4, 8, 3, 5, 6, 7]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]),
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
}
