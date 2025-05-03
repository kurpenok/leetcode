pub fn min_operations(logs: Vec<String>) -> i32 {
    logs.iter().fold(0, |depth, s| {
        if s.starts_with("..") {
            (depth - 1).max(0)
        } else if s.starts_with(".") {
            depth
        } else {
            depth + 1
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "../".to_string(),
                "d21/".to_string(),
                "./".to_string()
            ]),
            2
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "d2/".to_string(),
                "./".to_string(),
                "d3/".to_string(),
                "../".to_string(),
                "d31/".to_string()
            ]),
            3
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            min_operations(vec![
                "d1/".to_string(),
                "../".to_string(),
                "../".to_string(),
                "../".to_string()
            ]),
            0
        );
    }
}
