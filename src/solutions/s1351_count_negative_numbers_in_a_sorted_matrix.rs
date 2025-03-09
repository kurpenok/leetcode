pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    grid.iter().flatten().filter(|n| **n < 0).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
