pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
    let mut result: bool = false;
    let mut count: i32 = 0;

    if arr.len() < (m * k) as usize {
        return result;
    }

    'main: for i in 0..=(arr.len() - (m * k) as usize) {
        let mut prev = None;
        for subset in arr[i..].chunks_exact(m as usize) {
            let current = Some(subset);
            match prev == current {
                true => count += 1,
                false => count = 0,
            }

            if count + 1 == k {
                result = true;
                break 'main;
            }
            prev = current;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3), true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2), true);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3), false);
    }
}
