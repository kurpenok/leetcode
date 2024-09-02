use std::usize;

pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    if (r * c) as usize != mat.len() * mat[0].len() {
        return mat;
    }

    let mut elements: Vec<i32> = Vec::new();
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            elements.push(mat[i][j]);
        }
    }

    let mut new_matrix: Vec<Vec<i32>> = Vec::new();
    let mut k: usize = 0;
    for _i in 0..r {
        let mut new_line: Vec<i32> = Vec::new();
        for _j in 0..c {
            new_line.push(elements[k]);
            k += 1;
        }
        new_matrix.push(new_line);
    }

    new_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
            vec![vec![1, 2, 3, 4]],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
            vec![vec![1, 2], vec![3, 4]],
        );
    }
}
