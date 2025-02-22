pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
    let mut time: i32 = 0;

    let mut x: i32 = points[0][0];
    let mut y: i32 = points[0][1];

    for point in points {
        while x != point[0] || y != point[1] {
            if x != point[0] && y != point[1] {
                if x < point[0] && y < point[1] {
                    x += 1;
                    y += 1;
                } else if x < point[0] && y > point[1] {
                    x += 1;
                    y -= 1;
                } else if x > point[0] && y > point[1] {
                    x -= 1;
                    y -= 1;
                } else if x > point[0] && y < point[1] {
                    x -= 1;
                    y += 1;
                }
                time += 1;
            } else if x != point[0] && y == point[1] {
                time += (x - point[0]).abs();
                x = point[0];
            } else if x == point[0] && y != point[1] {
                time += (y - point[1]).abs();
                y = point[1];
            }
        }
    }

    time
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
            7
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
            5
        );
    }
}
