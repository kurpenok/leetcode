pub fn check_valid(matrix: Vec<Vec<i32>>) -> bool {
    let n = matrix.len();

    for i in 0..n {
        let mut row_mask = vec![false; n];
        let mut col_mask = vec![false; n];

        for j in 0..n {
            let row_val = matrix[i][j];
            let col_val = matrix[j][i];

            if row_val < 1 || row_val > n as i32 || col_val < 1 || col_val > n as i32 {
                return false;
            }

            let r_idx = (row_val - 1) as usize;
            let c_idx = (col_val - 1) as usize;

            if row_mask[r_idx] || col_mask[c_idx] {
                return false;
            }

            row_mask[r_idx] = true;
            col_mask[c_idx] = true;
        }
    }

    true
}
