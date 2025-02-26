use std::collections::HashSet;

pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut rows: HashSet<usize> = HashSet::new();
    let mut columns: HashSet<usize> = HashSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                rows.insert(i);
                columns.insert(j);
            }
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if rows.contains(&i) || columns.contains(&j) {
                matrix[i][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut matrix: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

        set_zeroes(&mut matrix);

        assert_eq!(matrix, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    }

    #[test]
    fn test_case_2() {
        let mut matrix: Vec<Vec<i32>> = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];

        set_zeroes(&mut matrix);

        assert_eq!(matrix, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);
    }
}
