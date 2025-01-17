pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; n as usize]; m as usize];

    for index in indices {
        let row = index[0] as usize;
        let col = index[1] as usize;

        for i in 0..matrix.len() {
            matrix[i][col] += 1;
        }

        for j in 0..matrix[0].len() {
            matrix[row][j] += 1;
        }
    }

    matrix
        .iter()
        .flatten()
        .filter(|&&cell| cell % 2 != 0)
        .count() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
    }
}
