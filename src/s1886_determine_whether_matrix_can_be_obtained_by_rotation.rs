fn rotate_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut rotated_matrix = vec![vec![0; matrix.len()]; matrix.len()];

    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            rotated_matrix[j][matrix.len() - 1 - i] = matrix[i][j];
        }
    }

    rotated_matrix
}

pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let mut matrix = mat;

    for _ in 0..4 {
        if matrix == target {
            return true;
        }
        matrix = rotate_matrix(matrix);
    }

    false
}
