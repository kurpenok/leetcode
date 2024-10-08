pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
    let mut projection = 0;
    let mut max_by_columns = vec![0; grid.len()];

    for row in &grid {
        let mut max_by_row = 0;
        for (column, &value) in row.iter().enumerate().filter(|(_, value)| **value > 0) {
            max_by_row = max_by_row.max(value);
            max_by_columns[column] = max_by_columns[column].max(value);
            projection += 1;
        }
        projection += max_by_row;
    }

    projection + max_by_columns.iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(projection_area(vec![vec![2]]), 5);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
    }
}
