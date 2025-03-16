use std::collections::HashSet;

pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let min_by_rows: Vec<i32> = matrix
        .iter()
        .map(|row| *row.iter().min().unwrap())
        .collect();

    let col_count = matrix[0].len();
    let mut max_by_columns = vec![i32::MIN; col_count];
    for row in &matrix {
        for (j, &val) in row.iter().enumerate() {
            if val > max_by_columns[j] {
                max_by_columns[j] = val;
            }
        }
    }

    let max_set: HashSet<_> = max_by_columns.into_iter().collect();
    min_by_rows
        .into_iter()
        .filter(|x| max_set.contains(x))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
            vec![15]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            lucky_numbers(vec![
                vec![1, 10, 4, 2],
                vec![9, 3, 8, 7],
                vec![15, 16, 17, 12]
            ]),
            vec![12]
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(lucky_numbers(vec![vec![7, 8], vec![1, 2]]), vec![7]);
    }
}
