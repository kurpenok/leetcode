pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    for i in 0..n {
        for j in 0..i {
            if i != j {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
    }

    for i in 0..n {
        for j in 0..n / 2 {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[i][n - 1 - j];
            matrix[i][n - 1 - j] = temp;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut image: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        rotate(&mut image);

        assert_eq!(image, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
    }

    #[test]
    fn test_case_2() {
        let mut image: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];

        rotate(&mut image);

        assert_eq!(
            image,
            [
                [15, 13, 2, 5],
                [14, 3, 4, 1],
                [12, 6, 8, 9],
                [16, 7, 10, 11]
            ]
        );
    }
}
