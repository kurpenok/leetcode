pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut triangle = triangle;

    for y in (0..triangle.len() - 1).rev() {
        for x in 0..triangle[y].len() {
            triangle[y][x] += std::cmp::min(triangle[y + 1][x], triangle[y + 1][x + 1]);
        }
    }

    triangle[0][0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(minimum_total(vec![vec![-10]]), -10);
    }
}
