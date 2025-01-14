pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    if coordinates.len() == 2 {
        true
    } else {
        for i in 2..coordinates.len() {
            let x_1 = coordinates[i - 2][0];
            let y_1 = coordinates[i - 2][1];

            let x_2 = coordinates[i - 1][0];
            let y_2 = coordinates[i - 1][1];

            let x_3 = coordinates[i][0];
            let y_3 = coordinates[i][1];

            if x_3 * (y_2 - y_1) - y_3 * (x_2 - x_1) != x_1 * y_2 - x_2 * y_1 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            check_straight_line(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 7],
            ]),
            true,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            check_straight_line(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![7, 7],
            ]),
            false,
        );
    }
}
