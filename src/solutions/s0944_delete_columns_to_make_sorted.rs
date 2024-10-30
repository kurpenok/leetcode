pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let mut deleted_count: i32 = 0;

    let mut strs_chars: Vec<Vec<char>> = Vec::new();
    for str in &strs {
        strs_chars.push(str.chars().collect::<Vec<char>>());
    }

    for i in 0..strs[0].len() {
        let mut prev = strs_chars[0][i];
        for str_chars in &strs_chars {
            if str_chars[i] < prev {
                deleted_count += 1;
                break;
            } else {
                prev = str_chars[i];
            }
        }
    }

    deleted_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            min_deletion_size(vec![
                "cba".to_string(),
                "daf".to_string(),
                "ghi".to_string()
            ]),
            1
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(min_deletion_size(vec!["a".to_string(), "b".to_string()]), 0);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            min_deletion_size(vec![
                "zyx".to_string(),
                "wvu".to_string(),
                "tsr".to_string()
            ]),
            3
        );
    }
}
