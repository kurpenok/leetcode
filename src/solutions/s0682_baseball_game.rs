pub fn cal_points(operations: Vec<String>) -> i32 {
    let mut points: Vec<i32> = Vec::new();

    for operation in operations {
        let try_number = operation.parse::<i32>();

        if try_number.is_ok() {
            points.push(try_number.unwrap());
        } else if operation == "+" {
            points.push(points[points.len() - 1] + points[points.len() - 2]);
        } else if operation == "D" {
            points.push(points[points.len() - 1] * 2);
        } else if operation == "C" {
            points.remove(points.len() - 1);
        }
    }

    points.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            cal_points(vec![
                "5".to_string(),
                "2".to_string(),
                "C".to_string(),
                "D".to_string(),
                "+".to_string()
            ]),
            30
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            cal_points(vec![
                "5".to_string(),
                "-2".to_string(),
                "4".to_string(),
                "C".to_string(),
                "D".to_string(),
                "9".to_string(),
                "+".to_string(),
                "+".to_string()
            ]),
            27
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(cal_points(vec!["1".to_string(), "C".to_string()]), 0);
    }
}
