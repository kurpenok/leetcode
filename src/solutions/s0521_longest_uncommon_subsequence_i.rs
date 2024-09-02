fn find_uncommon_subsequence_length(a: String, b: String) -> i32 {
    if a.len() != b.len() {
        return std::cmp::max(a.len(), b.len()) as i32;
    } else if a != b {
        return a.len() as i32;
    }

    let mut max_uncommon_subsequence_length: i32 = -1;

    for i in 0..=a.len() {
        let mut uncommon_subsequence_length: i32 = -1;

        for j in i..=a.len() {
            if i == j {
                continue;
            }

            let slice = &a[i..j];
            if !b.contains(slice) {
                uncommon_subsequence_length = slice.len() as i32;
            } else {
                break;
            }
        }

        max_uncommon_subsequence_length =
            std::cmp::max(max_uncommon_subsequence_length, uncommon_subsequence_length);
    }

    max_uncommon_subsequence_length
}

pub fn find_lu_slength(a: String, b: String) -> i32 {
    std::cmp::max(
        find_uncommon_subsequence_length(a.clone(), b.clone()),
        find_uncommon_subsequence_length(b.clone(), a.clone()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_lu_slength("aba".to_string(), "cdc".to_string()), 3);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_lu_slength("aaa".to_string(), "bbb".to_string()), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_lu_slength("aaa".to_string(), "aaa".to_string()), -1);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            find_lu_slength(
                "aefawfawfawfaw".to_string(),
                "aefawfeawfwafwaef".to_string()
            ),
            17
        );
    }
}
