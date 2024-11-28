pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
    let x_1 = points[0][0] as f64;
    let y_1 = points[0][1] as f64;
    let x_2 = points[1][0] as f64;
    let y_2 = points[1][1] as f64;
    let x_3 = points[2][0] as f64;
    let y_3 = points[2][1] as f64;

    (0.5 * (x_1 * (y_2 - y_3) + x_2 * (y_3 - y_1) + x_3 * (y_1 - y_2))) != 0.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
            false,
        );
    }
}
