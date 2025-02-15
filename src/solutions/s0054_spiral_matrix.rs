pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut spiral: Vec<i32> = Vec::new();
    let m: usize = matrix.len();
    let n: usize = matrix[0].len();

    let mut top: i32 = 0;
    let mut bottom: i32 = m as i32 - 1;
    let mut left: i32 = 0;
    let mut right: i32 = n as i32 - 1;

    while top <= bottom && left <= right {
        for j in left..=right {
            spiral.push(matrix[top as usize][j as usize]);
        }
        top += 1;

        for i in top..=bottom {
            spiral.push(matrix[i as usize][right as usize]);
        }
        right -= 1;

        if top <= bottom {
            for j in (left..=right).rev() {
                spiral.push(matrix[bottom as usize][j as usize]);
            }
            bottom -= 1;
        }

        if left <= right {
            for i in (top..=bottom).rev() {
                spiral.push(matrix[i as usize][left as usize]);
            }
            left += 1;
        }
    }

    spiral
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            [1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }
}
