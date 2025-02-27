pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    match matrix
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<i32>>()
        .binary_search(&target)
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}
