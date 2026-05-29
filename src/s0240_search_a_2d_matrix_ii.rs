pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    for i in 0..matrix.len() {
        if matrix[i][0] > target {
            return false;
        }

        if matrix[i].binary_search(&target).is_ok() {
            return true;
        }
    }

    false
}
