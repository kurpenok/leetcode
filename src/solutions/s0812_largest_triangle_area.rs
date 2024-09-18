pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
    let mut max_area = 0.0;

    for i in 0..points.len() {
        for j in i..points.len() {
            for k in j..points.len() {
                let a = points[j][0] - points[i][0];
                let b = points[k][1] - points[i][1];
                let c = points[k][0] - points[i][0];
                let d = points[j][1] - points[i][1];
                let area = 0.5 * ((a * b) - (c * d)).abs() as f64;
                if max_area < area {
                    max_area = area;
                }
            }
        }
    }

    max_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            largest_triangle_area(vec![
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]),
            2.0
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]),
            0.5
        );
    }
}
