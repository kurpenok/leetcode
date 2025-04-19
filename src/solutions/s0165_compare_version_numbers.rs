pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut version_1_by_point: Vec<i32> = version1
        .split(".")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let mut version_2_by_point: Vec<i32> = version2
        .split(".")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let maximum_len: usize = std::cmp::max(version_1_by_point.len(), version_2_by_point.len());
    for _ in 0..maximum_len - version_1_by_point.len() {
        version_1_by_point.push(0);
    }
    for _ in 0..maximum_len - version_2_by_point.len() {
        version_2_by_point.push(0);
    }

    for i in 0..maximum_len {
        if version_1_by_point[i] < version_2_by_point[i] {
            return -1;
        } else if version_1_by_point[i] > version_2_by_point[i] {
            return 1;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(compare_version("1.2".to_string(), "1.10".to_string()), -1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(compare_version("1.01".to_string(), "1.001".to_string()), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(compare_version("1.0".to_string(), "1.0.0.0".to_string()), 0);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            compare_version("7.5.2.4".to_string(), "7.5.3".to_string()),
            -1
        );
    }
}
